use eframe::egui;

struct Total {
    total_rendimento: f64,
    total_investido: f64,
    total_juros: f64,
}

fn calcular(valor_inicial: f64, taxa_juros: f64, anos: u32, investimento_mensal: f64) -> Total {
    let mut total_investido = valor_inicial;
    let mut total_rendimento = valor_inicial;

    let taxa_mensal = (1.0 + taxa_juros / 100.0).powf(1.0 / 12.0) - 1.0;

    for _ in 0..(anos * 12) {
        total_rendimento *= 1.0 + taxa_mensal;

        total_rendimento += investimento_mensal;

        total_investido += investimento_mensal;
    }

    let total_juros = total_rendimento - total_investido;

    Total {
        total_rendimento,
        total_investido,
        total_juros,
    }
}

fn converter_valor(valor: &str) -> Result<f64, String> {
    let numero = valor.trim().replace(',', ".");

    let resultado = numero
        .parse::<f64>()
        .map_err(|_| format!("Valor inválido: {}", valor))?;

    if !resultado.is_finite() || resultado < 0.0 {
        return Err("Informe um número válido e não negativo.".to_string());
    }

    Ok(resultado)
}

#[derive(Default)]
struct CalculadoraJurosCompostosApp {
    valor_inicial: String,
    taxa_juros: String,
    anos: String,
    investimento_mensal: String,
    resultado: Option<Total>,
    erro: Option<String>,
}

impl CalculadoraJurosCompostosApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }

    fn executar_calculo(&mut self) -> Result<Total, String> {
        let valor_inicial = converter_valor(&self.valor_inicial)?;
        let taxa_juros = converter_valor(&self.taxa_juros)?;
        let investimento_mensal = converter_valor(&self.investimento_mensal)?;

        let anos = self
            .anos
            .trim()
            .parse::<u32>()
            .map_err(|_| "Informe um número válido de anos.".to_string())?;

        if anos > 100 {
            return Err("O período máximo é de 100 anos.".to_string());
        }

        let total = calcular(valor_inicial, taxa_juros, anos, investimento_mensal);

        if !total.total_rendimento.is_finite() {
            return Err("O resultado excedeu o limite numérico.".to_string());
        }

        Ok(total)
    }
}

impl eframe::App for CalculadoraJurosCompostosApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("Calculadora de Juros Compostos");

            ui.add_space(10.0);

            egui::Grid::new("campos_calculadora")
                .num_columns(2)
                .spacing([15.0, 10.0])
                .show(ui, |ui| {
                    ui.label("Valor inicial (R$):");
                    ui.text_edit_singleline(&mut self.valor_inicial);
                    ui.end_row();

                    ui.label("Taxa de juros anual (%):");
                    ui.text_edit_singleline(&mut self.taxa_juros);
                    ui.end_row();

                    ui.label("Período (anos):");
                    ui.text_edit_singleline(&mut self.anos);
                    ui.end_row();

                    ui.label("Investimento mensal (R$):");
                    ui.text_edit_singleline(&mut self.investimento_mensal);
                    ui.end_row();
                });

            ui.add_space(10.0);
            ui.separator();

            if ui.button("Calcular").clicked() {
                match self.executar_calculo() {
                    Ok(resultado) => {
                        self.resultado = Some(resultado);
                        self.erro = None;
                    }
                    Err(mensagem) => {
                        self.resultado = None;
                        self.erro = Some(mensagem);
                    }
                }
            }

            if let Some(erro) = &self.erro {
                ui.colored_label(egui::Color32::RED, erro);
            }

            if let Some(resultado) = &self.resultado {
                ui.separator();

                ui.heading("Resultado");

                ui.label(format!(
                    "Total investido: R$ {:.2}",
                    resultado.total_investido
                ));

                ui.label(format!(
                    "Montante final: R$ {:.2}",
                    resultado.total_rendimento
                ));

                ui.label(format!("Total de juros: R$ {:.2}", resultado.total_juros));
            }
        });
    }
}

fn main() {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_resizable(false),
        ..Default::default()
    };

    eframe::run_native(
        "Calculadora de Juros Compostos",
        native_options,
        Box::new(|cc| Ok(Box::new(CalculadoraJurosCompostosApp::new(cc)))),
    )
    .expect("Falha ao iniciar a aplicação");
}
