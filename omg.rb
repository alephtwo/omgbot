require 'discordrb'

token = 'B0T.T0KEN.here'
bot = Discordrb::Commands::CommandBot.new token: token, prefix: '!'

puts "This bot's invite URL is #{bot.invite_url}."
puts 'Click on it to invite it to your server.'

bot.command :omg do |event|
  channel = event.user.voice_channel
  return unless channel

  # TODO: Block until after we've disconnected
  voice_bot = bot.voice_connect channel
  voice_bot.play_file File.join __DIR__, 'sounds', 'michiru.mp3'
  voice_bot.destroy
  nil
end

bot.run
