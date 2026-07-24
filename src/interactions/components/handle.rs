use super::dismiss::handle_dismiss;
use crate::{errors::StarboardResult, interactions::context::ComponentCtx};

pub async fn handle_component(ctx: ComponentCtx) -> StarboardResult<()> {
    let Some(id) = ctx.data.custom_id.strip_prefix("stateless::") else {
        return Ok(());
    };

    match id {
        "dismiss_notification" => handle_dismiss(&ctx).await?,
        _ => unreachable!("Unhandled stateless component: {id}"),
    }

    Ok(())
}
