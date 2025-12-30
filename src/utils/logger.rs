use fern::colors::{Color, ColoredLevelConfig};
use log::{info, warn, error, debug};
use std::time::SystemTime;

/// 初始化日志系统
pub fn init_logger() -> Result<(), fern::InitError> {
    let colors = ColoredLevelConfig::new()
        .debug(Color::Magenta)
        .info(Color::Green)
        .warn(Color::Yellow)
        .error(Color::Red);

    fern::Dispatch::new()
        // 日志格式
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{}] [{}] [{}] {}",
                humantime::format_rfc3339_seconds(SystemTime::now()),
                record.target(),
                colors.color(record.level()),
                message
            ));
        })
        // 输出到控制台
        .chain(std::io::stdout())
        // 日志级别
        .level(log::LevelFilter::Info)
        // 特定模块的日志级别
        .level_for("actix_web", log::LevelFilter::Warn)
        .level_for("diesel", log::LevelFilter::Warn)
        // 应用日志配置
        .apply()?;

    Ok(())
}


