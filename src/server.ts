import * as Discord from 'discord.js';

const client = new Discord.Client();

client.on('ready', () => {
    console.log(`Logged in as ${client!.user!.tag}!`);
});

client.on('message', async msg => {
    if (!msg.guild) return;
    if (msg.content === 'ping') {
        if (msg.member?.voice.channel) {
            const conn = await msg.member.voice.channel.join();
            const dispatcher = conn.play('/home/ben/repos/omgbot/sounds/clarisse/scene_evt170430_cp6_q2_s30_18.mp3');
            dispatcher.on('finish', () => {
                dispatcher.destroy();
                conn.disconnect();
            });
        }
    }
});

client.login(process.env.DISCORD_TOKEN);