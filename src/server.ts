import * as Discord from "discord.js";
import { Events, GatewayIntentBits } from "discord.js";
import { onMessageCreate } from "./events/onMessageCreate";
import { onReady } from "./events/onReady";
import { playSound } from "./sound/playSound";
import { pickSound } from "./sound/soundUtils";

const client = new Discord.Client({
  intents: [
    GatewayIntentBits.Guilds,
    GatewayIntentBits.GuildMessages,
    GatewayIntentBits.GuildVoiceStates,
    GatewayIntentBits.MessageContent,
  ],
});

client.on(Events.ClientReady, onReady(client));
client.on(Events.MessageCreate, onMessageCreate);

// This code is extremely cursed.
// It is cursed code.
// Remove it or keep it at your own peril.
client.on(Events.VoiceStateUpdate, (prev, next) => {
  // Ignore when the bot itself joins the channel to prevent duplicates.
  if (next.member?.id === client.user?.id) {
    return;
  }

  if (!prev.channel && next.channel) {
    const channel = next.channel;
    // Set a timeout on this so it doesn't play THE INSTANT the user joins
    setTimeout(() => {
      const sound = pickSound("greeting");
      playSound(channel, sound);
    }, 750);
  }
});

void client.login(process.env.DISCORD_TOKEN);
