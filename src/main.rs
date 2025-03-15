//! Módulo principal para integração com a API OpenAI Vision
//!
//! Este módulo fornece funcionalidade para processar imagens locais e interagir
//! com a API OpenAI Vision para análise de imagens. Utiliza o modelo GPT-4o-mini
//! para gerar descrições de imagens.

use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use dotenv::dotenv;
use std::error::Error;
use std::fs;

use async_openai::{
    Client,
    types::{
        ChatCompletionRequestMessageContentPartImageArgs,
        ChatCompletionRequestMessageContentPartTextArgs, ChatCompletionRequestUserMessageArgs,
        CreateChatCompletionRequestArgs, ImageDetail, ImageUrlArgs,
    },
};

/// Função principal que demonstra a integração com a API OpenAI Vision.
///
/// # Funcionalidade
/// - Carrega variáveis de ambiente do ficheiro .env
/// - Lê e codifica uma imagem local em base64
/// - Envia a imagem para a API OpenAI Vision
/// - Processa e exibe a resposta da API
///
/// # Erros
/// Retorna um erro se:
/// - O ficheiro .env não puder ser carregado
/// - A imagem não puder ser lida
/// - Houver falha na comunicação com a API
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Inicializa as variáveis de ambiente do ficheiro .env
    dotenv().ok();
    let client = Client::new();

    // Lê e codifica a imagem local em base64
    let image_path = "/Users/marioantunes/Desktop/Rust-LLM/manga.png"; // Substitua pelo caminho da sua imagem
    let image_data = fs::read(image_path)?;
    let base64_image = BASE64.encode(image_data);
    let image_url = format!("data:image/jpeg;base64,{}", base64_image);

    println!("Inicio do processamento da imagem");
    // quero sabe aunato tempo leva a API a responder
    let start = std::time::Instant::now();

    // Configura e constrói o pedido para a API Vision
    let request = CreateChatCompletionRequestArgs::default()
        .model("gpt-4o-mini")
        .max_tokens(1024_u32)
        .temperature(0.3)
        .top_p(0.7)
        .messages([ChatCompletionRequestUserMessageArgs::default()
            .content(vec![
                ChatCompletionRequestMessageContentPartTextArgs::default()
                    .text("imagem e banda desenha manga, traduz os baloes de texto para pt-pt e usa numeração, verifica dentro do mesmo quadro da esquerda para direita e do topo para baixo para eu saber a posição de cada balão")
                    .build()?
                    .into(),
                ChatCompletionRequestMessageContentPartImageArgs::default()
                    .image_url(
                        ImageUrlArgs::default()
                            .url(image_url)
                            .detail(ImageDetail::High)
                            .build()?,
                    )
                    .build()?
                    .into(),
            ])
            .build()?
            .into()])
        .build()?;

    // Imprime o pedido formatado em JSON
    //println!("{}", serde_json::to_string(&request).unwrap());

    // Envia o pedido e processa a resposta
    let response = client.chat().create(request).await?;

    // Calcula e exibe o tempo de resposta
    let duration = start.elapsed();

    // Exibe a resposta formatada
    if let Some(choice) = response.choices.first() {
        if let Some(content) = &choice.message.content {
            println!("{}", content);
            // formatar o tempo de resposta para minutos e segundos e ms
            let minutes = duration.as_secs() / 60;
            let seconds = duration.as_secs() % 60;
            let milliseconds = duration.subsec_millis();
            // milisegundos so com 3 digitos
            println!(
                "Tempo de resposta: {}'m, {}'s e {:03}'ms",
                minutes, seconds, milliseconds
            );
        }
    }
    Ok(())
}
