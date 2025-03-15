# Integração de OCR e Visão por IA em Rust

Este projeto em Rust demonstra a comparação entre tecnologias de OCR (Reconhecimento Ótico de Caracteres) e modelos de linguagem com capacidades de visão (LLM Vision) para processamento e análise de imagens. O foco principal é avaliar o desempenho e eficácia de ambas as abordagens para extração e tradução de texto em imagens, especialmente em mangas.

## Funcionalidades

- Processamento de imagens e codificação base64
- Integração com API OCR.space para reconhecimento de texto
- Integração com OpenAI Vision API (modelo GPT-4o-mini)
- Tradução de texto em mangas para português de Portugal
- Medição de desempenho e tempo de resposta
- Configuração via variáveis de ambiente

## Comparação OCR vs LLM Vision

O projeto permite comparar:
- Precisão na extração de texto de imagens
- Tempo de resposta de cada tecnologia
- Qualidade da tradução para português de Portugal
- Capacidade de compreensão contextual das imagens

## Pré-requisitos

- Rust (versão estável mais recente)
- Chave de API OpenAI
- Chave de API OCR.space

## Sites

- https://platform.openai.com/api-keys
- https://ocr.space/OCRAPI

## Configuração

1. Clone o repositório:
   ```bash
   git clone <url-do-repositório>
   cd Rust-LLM
   ```

2. Crie um ficheiro `.env` na raiz do projeto e adicione as suas chaves de API:
   ```
   OPENAI_API_KEY=sua_chave_api_openai
   OCR_API_KEY=sua_chave_api_ocr
   ```

3. Instale as dependências:
   ```bash
   cargo build
   ```

## Utilização

1. Coloque os seus ficheiros de imagem na diretoria do projeto

2. Atualize os caminhos das imagens em `src/main.rs` conforme necessário:
   ```rust
   let image_path = "caminho/para/sua/imagem.png";
   ```

3. Execute o projeto:
   ```bash
   cargo run
   ```

## Dependências

- `async-openai`: Cliente para API OpenAI
- `reqwest`: Cliente HTTP para API OCR
- `base64`: Codificação de imagens
- `dotenv`: Gestão de variáveis de ambiente
- `tokio`: Runtime assíncrono
- `serde_json`: Serialização JSON

## Tratamento de Erros

A aplicação trata vários casos de erro:
- Falhas no carregamento do ficheiro de ambiente
- Erros na leitura de ficheiros de imagem
- Problemas de comunicação com as APIs

## Desempenho

A aplicação inclui monitorização de desempenho, exibindo tempos de resposta em minutos, segundos e milissegundos para chamadas de API, permitindo uma comparação direta entre as tecnologias OCR e LLM Vision.

## Licença

Este projeto é de código aberto e disponível sob a Licença MIT.