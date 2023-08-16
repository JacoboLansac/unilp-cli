use std::env;

#[derive(Debug)]
enum Sentiment {
    Bullish,
    Bearish,
    Neutral,
}

#[derive(Debug)]
enum Timeframe {
    Day,
    Week,
    Month,
    Year,
}

#[derive(Debug)]
struct LPposition {
    min_price: f64,
    max_price: f64,
}

fn parse_sentiment(sent_str: &str) -> Result<Sentiment, &str> {
    match sent_str {
        "bullish" => Ok(Sentiment::Bullish),
        "bearish" => Ok(Sentiment::Bearish),
        "neutral" => Ok(Sentiment::Neutral),
        _ => Err("Invalid sentiment"),
    }
}

fn parse_timeframe(tframe_str: &str) -> Result<Timeframe, &str> {
    match tframe_str {
        "d" | "day" => Ok(Timeframe::Day),
        "w" | "week" => Ok(Timeframe::Week),
        "m" | "month" => Ok(Timeframe::Month),
        "y" | "year" => Ok(Timeframe::Year),
        _ => Err("Invalid Timeframe"),
    }
}

fn get_r_value(timeframe: &Timeframe) -> f64 {
    match timeframe {
        Timeframe::Day => 1.065,
        Timeframe::Week => 1.175,
        Timeframe::Month => 1.4,
        Timeframe::Year => 3.25,
    }
}

fn get_lp_position(current_price: f64, sentiment: Sentiment, r: f64) -> LPposition {
    match sentiment {
        Sentiment::Bearish => LPposition {
            min_price: (current_price / r.powi(2)),
            max_price: (current_price),
        },
        Sentiment::Neutral => LPposition {
            min_price: (current_price / r),
            max_price: (current_price * r),
        },
        Sentiment::Bullish => LPposition {
            min_price: (current_price),
            max_price: (current_price * r.powi(2)),
        },
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let current_price: f64 = (args[1])
        .parse::<f64>()
        .expect("First script argument should be the price (float)");

    let sent_str: &str = &args[2];
    let tframe_str: &str = &args[3];

    let sentiment: Sentiment = parse_sentiment(sent_str).expect("Invalid Sentiment");
    let timeframe: Timeframe = parse_timeframe(tframe_str).expect("Invalid timeframe");

    println!("sentiment: {:?}", sentiment);
    println!("timeframe: {:?}", timeframe);
    println!("current price: {}", current_price);

    let r = get_r_value(&timeframe);
    let lp_position = get_lp_position(current_price, sentiment, r);

    println!("{:?}", lp_position);
}
