# OpenUtau Phonemizer Studio & C# Compiler (Rust)

Um software completo, profissional, nativo e de alta performance desenvolvido em **Rust** para projetar, configurar, simular e compilar **Phonemizers completos em C# (.dll e .cs)** para o [OpenUtau](https://github.com/openutau/OpenUtau).

---

## Funcionalidades Principais

1. **OpenUtau GitHub Hub & Importador C#**:
   - Conecta-se à API do GitHub e lista todos os phonemizers oficiais do repositório OpenUtau (`OpenUtau.Plugin.Builtin`).
   - Importa código fonte C# de qualquer repositório GitHub, URL direta ou arquivo `.cs` local.
   - **Parser e Engenharia Reversa**: Reconhece automaticamente atributos `[Phonemizer]`, listas de vogais, consoantes, clusters, dicionários e regras Regex, convertendo tudo para o projeto editável.
   - **Modo C# Puro Direto**: Permite colar e editar código C# arbitrário diretamente no estúdio e compilar imediatamente para DLL.

2. **Editor Fonético & Fonológico Ultra-Completo**:
   - **Vogais e IPA**: Símbolos IPA, orais, nasais, ditongos e tritongos com controle de divisão percentual (ex: 65%/35%), glides de finalização (`a -`, `a R`, `a h`, `a AP`) e ataques glotais (`' a`, `? a`).
   - **Consoantes e Ponto/Modo de Articulação**:
     - *Modos*: Oclusivas, Fricativas, Nasais, Líquidas, Africadas, Semivogais, Glotais, Clicks/Ejetivas, Especiais.
     - *Pontos*: Bilabial, Labiodental, Dental, Alveolar, Postalveolar, Retroflexo, Palatal, Velar, Uvular, Faríngeo, Glotal.
     - *Propriedades*: Sonora/Surda, Aspirada, Onset, Coda, Intervocálica, Multiplicador de timing e Pré-emissão.
   - **Encontros Consonantais (Clusters)**: Suporte a onset e coda, divisão proporcional, limites de BPM e elisão automática em andamentos rápidos.
   - **Dicionário & Exceções**: Importação e exportação de dicionários em formato **CMUdict**, **TXT**, **TSV** e **CSV**.
   - **Importação em Lote & Filtros**: Busca rápida em tempo real em todas as tabelas e assistente de importação rápida de múltiplos fonemas.

3. **Matriz Fonética, Voice Colors & Fonotática**:
   - Concatenações suportadas: `- V`, `- CV`, `VCV`, `CV`, `VC`, `VV`, `CC`, `CVC`, `V -`, `C -`, `' V`, `V ɾ V`, `C_V` (Liaison), respirações (`br`) e pausas.
   - Filtros de Pitch (faixa de notas) e filtros por Voice Color (`Power`, `Soft`, etc.).
   - Suporte a Fonotática com Liaison e ligação entre palavras.

4. **Bancada de Testes & Simulador Fonético em Tempo Real**:
   - Digite qualquer letra ou frase musical e visualize a decomposição em tempo real das notas e regras aplicadas antes de compilar.

5. **Compilador Integrado de `.DLL`**:
   - Compilação via .NET SDK (`dotnet build`).
   - Deploy automático com 1 clique direto na pasta `Plugins/` do seu OpenUtau.
   - Streaming assíncrono de logs de compilação em tempo real.

6. **9 Presets Embutidos + Criador Universal Do Zero**:
   - **Universal Do Zero (Custom)**: Crie qualquer idioma ou sistema fonético customizado do zero.
   - **Português Brasileiro (PT-BR)**
   - **Japonês VCV (JA-VCV)**
   - **Espanhol (ES-ES)**
   - **Francês (FR-FR)**
   - **Russo CVC (RU-CVC)**
   - **Coreano Hangul (KO-KR)**
   - **Chinês Mandarim (ZH-CN)**
   - **Inglês Arpasing (EN-ARPA)**

---

## Como Executar

### 1. Abrir a Interface Gráfica Desktop (Nativa GPU)
```bash
cargo run --release
```

### 2. Compilar via Linha de Comando (CLI)
```bash
./target/release/phonemizer-studio --build presets_json/universal_scratch.json ./test_plugins_output
```

### 3. Executar Testes Automatizados
```bash
cargo test
```
