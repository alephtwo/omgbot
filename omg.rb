require 'discordrb'

token = ARGV[0]
bot = Discordrb::Commands::CommandBot.new token: token, prefix: '!'
puts "This bot's invite URL is #{bot.invite_url}."

sounds = File.join __dir__, 'sounds'
voice_channel_lock = Mutex.new

play_sound = lambda do |sound|
  lambda do |event|
    channel = event.user.voice_channel
    return unless channel

    # Needs to be synchronized, otherwise we'll try to do too much at once.
    voice_channel_lock.synchronize do
      voice_bot = bot.voice_connect channel
      voice_bot.play_file sound
      voice_bot.destroy
    end
    nil
  end
end

bot.command :omg do |event|
  sound = Dir.glob(File.join(sounds, '*')).sample
  play_sound.call(sound).call(event)
end
bot.command :michiru, &play_sound.call(File.join(sounds, 'michiru.mp3'))
bot.run
