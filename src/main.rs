use eframe::egui::{self, Ui};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex, mpsc},
    thread,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Message {
    from: String,
    text: String,
}

type Peers = Arc<Mutex<HashMap<String, TcpStream>>>;

struct ChatApp {
    // Dados de rede
    my_addr: String,
    target_addr: String, // Para o campo de texto de conexão
    peers: Peers,

    // UI e Comunicação
    input_text: String,
    history: Vec<Message>,
    sender: mpsc::Sender<Message>, // Guardamos o sender para usar no botão conectar
    receiver: mpsc::Receiver<Message>,
}

impl ChatApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let (sender, receiver) = mpsc::channel();
        let peers: Peers = Arc::new(Mutex::new(HashMap::new()));

        // Iniciamos um servidor numa porta aleatória ou fixa para testes
        Self {
            my_addr: "127.0.0.1:8080".to_string(),
            target_addr: "127.0.0.1:8081".to_string(),
            peers,
            input_text: String::new(),
            history: Vec::new(),
            sender,
            receiver,
        }
    }

    // Função que inicia o servidor local
    fn start_server(&mut self, ctx: egui::Context) {
        let addr = self.my_addr.clone();
        let peers = self.peers.clone();
        let sender = self.sender.clone();

        thread::spawn(move || {
            if let Ok(listener) = TcpListener::bind(&addr) {
                for stream in listener.incoming().flatten() {
                    let p = peers.clone();
                    let s = sender.clone();
                    let c = ctx.clone();
                    thread::spawn(move || handle_connection(stream, p, s, c));
                }
            }
        });
    }
}

impl eframe::App for ChatApp {
    fn ui(&mut self, ui: &mut Ui, _frame: &mut eframe::Frame) {
        // Receber mensagens novas
        while let Ok(msg) = self.receiver.try_recv() {
            self.history.push(msg);
        }

        egui::Panel::top("server_panel").show_inside(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label("Minha Porta:");
                ui.text_edit_singleline(&mut self.my_addr);
                if ui.button("Iniciar Servidor").clicked() {
                    self.start_server(ui.ctx().clone());
                    self.history.push(Message {
                        from: "Sistema".into(),
                        text: "Servidor iniciado!".into(),
                    });
                }
            });

            ui.horizontal(|ui| {
                ui.label("Conectar em:");
                ui.text_edit_singleline(&mut self.target_addr);
                if ui.button("Conectar").clicked() {
                    connect_to(
                        &self.target_addr,
                        self.peers.clone(),
                        self.sender.clone(),
                        ui.ctx().clone(),
                    );
                }
            });
        });

        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.heading("Mensagens");
            ui.separator();

            egui::ScrollArea::vertical()
                .stick_to_bottom(true)
                .show(ui, |ui| {
                    for msg in &self.history {
                        ui.label(format!("{}: {}", msg.from, msg.text));
                    }
                });

            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                ui.horizontal(|ui| {
                    let res = ui.text_edit_singleline(&mut self.input_text);
                    if (ui.button("Enviar").clicked()
                        || (res.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter))))
                        && !self.input_text.is_empty()
                    {
                        let msg = Message {
                            from: self.my_addr.clone(),
                            text: self.input_text.clone(),
                        };
                        broadcast(&self.peers, &msg, None);
                        self.history.push(msg);
                        self.input_text.clear();
                    }
                });
            });
        });
    }
}

// --- Funções de Rede (idênticas à lógica anterior, mas chamadas pela UI) ---

fn handle_connection(
    stream: TcpStream,
    peers: Peers,
    sender: mpsc::Sender<Message>,
    ctx: egui::Context,
) {
    let peer_addr = stream.peer_addr().unwrap().to_string();
    peers
        .lock()
        .unwrap()
        .insert(peer_addr.clone(), stream.try_clone().unwrap());

    let reader = BufReader::new(stream);
    for line in reader.lines().flatten() {
        if let Ok(msg) = serde_json::from_str::<Message>(&line) {
            let _ = sender.send(msg);
            ctx.request_repaint();
        }
    }
    peers.lock().unwrap().remove(&peer_addr);
}

fn connect_to(addr: &str, peers: Peers, sender: mpsc::Sender<Message>, ctx: egui::Context) {
    let addr_str = addr.to_string();
    thread::spawn(move || {
        if let Ok(stream) = TcpStream::connect(&addr_str) {
            let s = stream.try_clone().unwrap();
            peers.lock().unwrap().insert(addr_str.clone(), s);
            handle_connection(stream, peers, sender, ctx);
        }
    });
}

fn broadcast(peers: &Peers, msg: &Message, skip: Option<&str>) {
    let json = serde_json::to_string(msg).unwrap() + "\n";
    let mut map = peers.lock().unwrap();
    map.retain(|addr, stream| {
        if skip.map_or(false, |s| s == addr) {
            return true;
        }
        stream.write_all(json.as_bytes()).is_ok()
    });
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "P2P Chat Dinâmico",
        options,
        Box::new(|cc| Ok(Box::new(ChatApp::new(cc)))),
    )
}
