mod markdown;
mod stream;

pub use self::markdown::{MarkdownRender, RenderOptions};
use self::stream::{markdown_stream, raw_stream};

use crate::utils::{error_text, AbortSignal};
use crate::{client::SseEvent, config::GlobalConfig};

use anyhow::Result;
use is_terminal::IsTerminal;
use std::io::stdout;
use tokio::sync::mpsc::UnboundedReceiver;

pub async fn render_stream(
    rx: UnboundedReceiver<SseEvent>,
    config: &GlobalConfig,
    abort: AbortSignal,
) -> Result<()> {
    if stdout().is_terminal() {
        let render_options = config.read().get_render_options()?;
        let mut render = MarkdownRender::init(render_options)?;
        markdown_stream(rx, &mut render, &abort).await
    } else {
        raw_stream(rx, &abort).await
    }
}

pub fn render_error(err: anyhow::Error, highlight: bool) {
    let err = format!("{err:?}");
    if highlight {
        eprintln!("{}", error_text(&err));
    } else {
        eprintln!("{err}");
    }
}
