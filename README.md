# 🐹 Kalorik  (Lite version) — Your Personal Nutrition Assistant Bot

![Kalorik banner](assets/kalorik-banner.png)

Kalorik is a smart Telegram bot powered by Rust + SQLx + OpenAI that helps you track your nutrition, calculate your daily intake, and improve your lifestyle — all in a friendly chat format.

## 🧠 Features

- 📊 **Calorie & Macronutrient Analysis** (via text, photos, or voice input)
- 📝 **Personal Profile** with age, weight, height, and goals
- 🧮 **Automatic BMI and BMR Calculation**
- 🎯 **Goal Tracking**: weight loss, maintenance, or gain
- 🖼️ **Image & Voice Recognition** powered by AI
- 📅 **History Logging** (daily food diary)
- 🐹 **Friendly UI** with cute mascot "Fit-Hamster"

## 📸 Screenshot

> 🐹 *"Just tell me what you ate — and I’ll do the rest!"*

```
🍽️ What did you eat today?
🥗 Caesar salad and grilled chicken
✅ 445 kcal, 38g protein, 18g fat, 21g carbs
```

## 🚀 Getting Started

### 1. Clone the Repo

```bash
git clone https://github.com/digkill/kalorik.git
cd kalorik
```

### 2. Setup Environment Variables

```env
DATABASE_URL=postgres://username:password@localhost:5432/kalorik_db
TELEGRAM_BOT_TOKEN=your_bot_token_here
OPENAI_API_KEY=your_openai_api_key
```

### 3. Run Migrations

```bash
cargo install sqlx-cli
sqlx migrate run
```

### 4. Run the Bot

```bash
cargo run
```

## ⚙️ Commands

| Command           | Description                          |
|------------------|--------------------------------------|
| `/start`          | Start the bot & register the user    |
| `/profile`        | Show or update your profile          |
| `/goal`           | Set your goal (lose, keep, gain)     |
| `/history`        | View your logged meals               |
| `/weight`         | Update your current weight           |
| `/help`           | Show available commands              |

## 🐹 Mascot: Fit-Hamster

> Our friendly Fit-Hamster 🐹 keeps you on track!  
> He logs your meals, reminds you of your goals, and sends you encouragements when you need it most.

![Kalorik Mascot](./assets/logo.png)

## 🧰 Tech Stack

- 🚀 Rust
- 💬 Teloxide
- 🧠 OpenAI API
- 🗃️ PostgreSQL + SQLx
- 🧪 Tokio async runtime
- 📦 Docker-ready

## 📄 License

MIT License © 2025 [Vitaliy Edifanov]
