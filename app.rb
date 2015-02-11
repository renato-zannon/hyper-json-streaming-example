require 'sinatra'
require 'json'

class Stream
  def each
    hash = { index: 0 }

    loop do
        hash[:index] += 1
        yield hash.to_json + "\r\n"
        sleep 0.5
    end
  end
end

get '/' do
  Stream.new
end
