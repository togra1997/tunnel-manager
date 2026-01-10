use eframe::egui;
use regex::Regex;
use std::fs::{File, create_dir_all};
use std::io::{BufRead, BufReader, Write}; // 正規表現ライブラリを使用

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // ロガーの初期化
    let options = eframe::NativeOptions::default();

    // アプリケーションの初期化時にポート番号をロード
    let mut app = MyApp::default();
    app.load_port("datas/latest_port.txt");
    app.load_adress("datas/latest_adress.txt");

    eframe::run_native(
        "Input and Execute",
        options,
        Box::new(|cc| {
            // 日本語フォントを設定
            set_japanese_font(&cc.egui_ctx);
            Ok(Box::new(app))
        }),
    )
}

// 日本語フォントを設定する関数
fn set_japanese_font(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    // 日本語フォントファイルを指定
    fonts.font_data.insert(
        "noto_jp".to_owned(),
        egui::FontData::from_static(include_bytes!("NotoSansJP-Regular.ttf")).into(),
    );

    // プロポーショナルフォントとモノスペースフォントに日本語フォントを設定
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "noto_jp".to_owned());
    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .insert(0, "noto_jp".to_owned());

    ctx.set_fonts(fonts);
}

#[derive(Default)]
struct MyApp {
    adress: String,
    server_port: String,
    command_output: String,
    command_error: String,
    port_error: String, // ポート入力エラーのメッセージ
    copy_command: String,
}

impl MyApp {
    // サーバーポートを保存する関数（最新のポートのみを保存）
    fn save_port(&self, path: &str) {
        if self.server_port.is_empty() {
            return;
        }

        // フォルダが存在しない場合は作成
        if let Some(parent) = std::path::Path::new(path).parent() {
            let _ = create_dir_all(parent);
        }

        // ポート番号を上書き保存（最新のポートのみを残す）
        if let Ok(mut file) = File::create(path) {
            let _ = writeln!(file, "{}", self.server_port);
        }
    }

    // サーバーポートをロードする関数
    fn load_port(&mut self, path: &str) {
        if let Ok(file) = File::open(path) {
            let reader = BufReader::new(file);
            if let Some(Ok(line)) = reader.lines().next() {
                self.server_port = line;
            }
        }
    }

    // 接続先アドレスを保存する関数（最新のアドレスのみを保存）
    fn save_adress(&self, path: &str) {
        if self.adress.is_empty() {
            return;
        }

        // フォルダが存在しない場合は作成
        if let Some(parent) = std::path::Path::new(path).parent() {
            let _ = create_dir_all(parent);
        }

        // アドレスを上書き保存（最新のアドレスのみを残す）
        if let Ok(mut file) = File::create(path) {
            let _ = writeln!(file, "{}", self.adress);
        }
    }

    // 接続先アドレスをロードする関数
    fn load_adress(&mut self, path: &str) {
        if let Ok(file) = File::open(path) {
            let reader = BufReader::new(file);
            if let Some(Ok(line)) = reader.lines().next() {
                self.adress = line;
            }
        }
    }

    // ポート番号のバリデーション
    fn validate_port(&mut self) -> bool {
        let re = Regex::new(r"^\d+$").unwrap(); // 半角数字のみ許可
        if self.server_port.is_empty() || re.is_match(&self.server_port) {
            self.port_error.clear(); // エラーなし
            true
        } else {
            self.port_error = "ポート番号は半角数字のみ入力してください".to_string();
            false
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("接続先アドレス:");
            ui.text_edit_singleline(&mut self.adress);

            ui.label("サーバーポート:");
            ui.text_edit_singleline(&mut self.server_port);

            // ポート入力エラーがあれば赤文字で表示
            if !self.port_error.is_empty() {
                ui.colored_label(egui::Color32::RED, &self.port_error);
            }

            if ui.button("実行").clicked() {
                // ポート番号をバリデート
                if self.validate_port() && !self.adress.is_empty() {
                    // コマンドを生成
                    self.command_output = format!(
                        "cloudflared access tcp --hostname {} --url localhost:{}",
                        self.adress, self.server_port,
                    );
                    self.copy_command = format!("localhost:{}", self.server_port);

                    // 最新のアドレスを保存
                    self.save_adress("datas/latest_adress.txt");

                    // 最新のポート番号を保存
                    self.save_port("datas/latest_port.txt");
                }
            }

            if !self.copy_command.is_empty() {
                ui.label("出力:");
                // コマンドを編集不可の形式で表示
                ui.label(&self.copy_command);

                // コピーボタンを追加
                if ui.button("コピー").clicked() {
                    ctx.copy_text(self.copy_command.clone());
                }
            }

            if !self.command_error.is_empty() {
                ui.label("エラー:");
                ui.monospace(&self.command_error);
            }
        });
    }
}
