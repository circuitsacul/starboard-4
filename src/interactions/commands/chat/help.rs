use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::channel::message::{
    Component,
    component::{ActionRow, Button, ButtonStyle},
};
use twilight_util::builder::embed::EmbedFieldBuilder;

use crate::{
    concat_format, constants, errors::StarboardResult, interactions::context::CommandCtx,
    utils::embed,
};

fn buttons() -> Vec<Component> {
    let link_btn = |name: &str, link: &str| {
        Component::Button(Button {
            sku_id: None,
            custom_id: None,
            disabled: false,
            emoji: None,
            label: Some(name.into()),
            style: ButtonStyle::Link,
            url: Some(link.into()),
            id: None,
        })
    };

    let buttons = vec![
        link_btn("Invite", constants::INVITE_URL),
        link_btn("Support", constants::SUPPORT_URL),
        link_btn("Documentation", constants::DOCS_URL),
        link_btn("Source", constants::SOURCE_URL),
        link_btn("Premium", constants::PATREON_URL),
    ];

    let row = Component::ActionRow(ActionRow {
        components: buttons,
        id: None,
    });

    vec![row]
}

#[derive(CommandModel, CreateCommand)]
#[command(name = "help", desc = "Get help with and general info for Starboard.")]
pub struct Help;

impl Help {
    pub async fn callback(self, mut ctx: CommandCtx) -> StarboardResult<()> {
        let emb = embed::build()
            .title("Starboard")
            .description("A pretty good starboard bot.")
            .field(EmbedFieldBuilder::new(
                "Getting Started",
                concat!(
                    "Get started by running `/starboards create` to create a starboard, and\n",
                    "then `/starboards edit requirements` to change requirement settings.\n",
                    "- `required`: how many upvotes a message needs\n",
                    "- `upvote-emojis`: what emojis count as upvotes\n",
                    "- `self-vote`: whether you can vote on your own messages\n\n",
                    "All other commands and settings are available in the docs."
                ),
            ))
            .field(EmbedFieldBuilder::new(
                "Features",
                concat!(
                    "- Multiple starboards\n",
                    "- Custom avatar/username for starboards (via webhooks)\n",
                    "- Autostar channels\n",
                    "- Complete per-channel starboard configuration\n",
                    "- Limited per-role starboard configuration\n",
                ),
            ))
            .field(EmbedFieldBuilder::new(
                "Starboard Premium",
                concat_format!(
                    "Premium works using a credit-based system. Each USD that you spend is ";
                    "equivalent to 1 premium credit. Once you have 3 credits, you can redeem it ";
                    "for 1 month of premium in any server.\n\n";
                    "Use `/premium credits` to see your credits, and `/premium redeem` to redeem ";
                    "premium in a server.\n\n";
                    "To get premium, visit [patreon.com]({})." <- constants::PATREON_URL;
                ),
            ))
            .build();

        ctx.respond(ctx.build_resp().embeds([emb]).components(buttons()).build())
            .await?;

        Ok(())
    }
}
