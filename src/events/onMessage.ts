import { Message } from "discord.js";
import { pickSound, getAllCategories } from '../sounds';

const categories = getAllCategories();

export default async (msg: Message) => {
    // If it's not from a guild, don't bother doing anything.
    if (!msg.guild) {
        return;
    }

    // If this isn't a command, we can stop.
    const categories = getAllCategories();
    if (!msg.content.startsWith('!')) {
        return;
    }

    // Make sure it's a real category
    const category = msg.content.replace(/^!/, '');
    if (category === 'help') {
        const help = Array.from(categories)
            .sort((a: string, b: string) => a.localeCompare(b))
            .map(c => `* !${c}`)

        msg.channel.send(help);
        return;
    }
    
    if (!categories.has(category)) {
        return;
    }

    // If the user isn't in a voice channel let's stop.
    if (!msg.member?.voice.channel) {
        return;
    }

    const conn = await msg.member.voice.channel.join();
    const sound = pickSound(category);

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