require 'discordrb'
require 'thread'

token = ARGV[0]
bot = Discordrb::Commands::CommandBot.new token: token, prefix: '!'
puts "This bot's invite URL is #{bot.invite_url}."

sounds = File.join __dir__, 'sounds'
voice_channel_lock = Mutex.new

def play_sound(event, file) do
  channel = event.user.voice_channel
  return unless channel

  # Needs to be synchronized, otherwise we'll try to do too much at once.
  voice_channel_lock.synchronize do
    voice_bot = bot.voice_connect channel
    voice_bot.play_file chosen_sound
    voice_bot.destroy
  end
  nil
end

bot.command :omg { |e| play_sound(e, Dir.glob(File.join(sounds, '*')).sample) }
bot.command :michiru { |e| play_sound(File.join(sounds, 'michiru.mp3')) }

bot.run
