import { mock } from "strong-mock";
import StatsCommand from "./StatsCommand";
import { Message } from "discord.js";

const message = mock<Message>();
const command = new StatsCommand(message);

command.run();
