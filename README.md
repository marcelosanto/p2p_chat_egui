# Rust P2P Chat com eframe/egui

Uma aplicação de chat Peer-to-Peer (P2P) com interface gráfica (GUI) desenvolvida em Rust. Este projeto é uma evolução de um sistema de chat via terminal para uma aplicação interativa moderna.

## 🚀 Funcionalidades

- **Interface Gráfica**: Interface intuitiva construída com `eframe` e `egui`.
- **Arquitetura P2P**: Comunicação direta entre nós sem necessidade de um servidor central.
- **Conexão Dinâmica**: Interface para configurar a porta local e conectar-se a endereços IP remotos em tempo real.
- **Async-ready**: Processamento de rede em threads separadas usando canais (`mpsc`) para garantir que a interface nunca bloqueie.
- **Protocolo JSON**: Troca de dados estruturada com `serde`.

## 🛠️ Como Executar

### Pré-requisitos
Certifica-te de que tens o Rust instalado ([rustup.rs](https://rustup.rs/)).

### Instalação e Execução
1. Clona este repositório ou copia os ficheiros.
2. No terminal, executa:

  ```bash
   cargo run
  ```

### Como testar (Localmente)

1.  Abre duas instâncias da aplicação.
    
2.  Na **Instância A**: Define "Minha Porta" como `8080` e clica em **Iniciar Servidor**.
    
3.  Na **Instância B**: Define "Minha Porta" como `8081` e clica em **Iniciar Servidor**.
    
4.  Na **Instância B**: No campo "Conectar em", digita `127.0.0.1:8080` e clica em **Conectar**.
    
5.  Agora podes trocar mensagens entre as duas janelas!
    

## 📚 Créditos

A lógica base de rede e a estrutura P2P deste projeto foram inspiradas e adaptadas a partir do tutorial de **Douglas Starnes**.

-   **Fonte Original**: [Peer-to-Peer Networking in Rust](https://www.youtube.com/watch?v=C9NC0RTCoT0) (YouTube).
    
-   **Autor Original**: Douglas Starnes.
    

A implementação atual expande o código original ao remover a dependência de argumentos de linha de comando e adicionar uma interface gráfica funcional com gestão de estado reativo.

## 📄 Licença

Este projeto é distribuído para fins educativos.

```

### O que foi incluído no README:
1.  **Título e Descrição**: Explica o que o projeto faz.
2.  **Funcionalidades**: Destaca o uso do `eframe` e da rede P2P.
3.  **Guia de Teste**: Um passo a passo prático para quem baixar o código saber como conectar dois chats no mesmo PC.
4.  **Secção de Créditos**: Menciona explicitamente o autor original e o link do vídeo do YouTube conforme solicitado.
5.  **Instruções de Compilação**: O comando padrão `cargo run`.
```
