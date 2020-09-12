import { Message } from "discord.js";
import { pickSound } from '../sounds';

export default async (msg: Message) => {
    // If it's not from a guild, don't bother doing anything.
    if (!msg.guild) {
        return;
    }

    // If this isn't a command, we can stop.
    if (!msg.content.startsWith('!')) {
        return;
    }

    // If the user isn't in a voice channel let's stop.
    if (!msg.member?.voice.channel) {
        return;
    }

    const conn = await msg.member.voice.channel.join();
    const sound = pickSound(msg.content.replace(/!/, ''));

    try {
        const dispatcher = conn.play(sound);
        dispatcher.on('finish', () => {
            dispatcher.destroy();
            conn.disconnect();
        });
    } catch (err) {
        conn.disconnect();
    }
}