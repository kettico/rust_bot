use crate::Data;
use tracing::error;

pub type Error = Box<dyn std::error::Error + Send + Sync>;

pub async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    // Custom Error Handler
    // This is for error we want to custom handler, rest get sent to deafaul handler
    match error {
        poise::FrameworkError::Setup { error, .. } => panic!("Failed to start bot: {:?}", error),
        poise::FrameworkError::Command { error, ctx, .. } => {
            error!("Error in command `{}`: {:?}", ctx.command().name, error);
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                error!("Error while handling error: {}", e)
            }
        }
    }
}
