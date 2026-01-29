local socket = require "socket"

local sock = assert(socket.udp4())
local ip = "239.255.255.250"
local port = 8888

local function get_my_addr()
  local hn = socket.dns.gethostname()
  local ip, add = socket.dns.toip(hn)
  if ip == "0.0.0.0" or ip == "127.0.0.1" then
    for _,ip in ipairs(add.ip) do
      if ip ~= "0.0.0.0" and ip ~= "127.0.0.1" then
        return ip
      end
    end 
  end
  return ip
end
local my_ip = get_my_addr()
assert(sock:setsockname("*", port))
assert(sock:setoption('reuseaddr', true))
assert(sock:setoption('ip-add-membership', {
  multiaddr = ip,
  interface = "0.0.0.0",
}))
assert(sock:setoption('ip-multicast-loop', true))
sock:settimeout(10)
local recvd = true
while true do
  if recvd then
    io.stdout:write("waiting for message")
    io.stdout:flush()
    sock:sendto("hi!", ip, port)
    recvd = false
  end
  local msg,who,who_port = sock:receivefrom()
  if not msg then
    if who == "timeout" then
      io.stdout:write(".")
      io.stdout:flush()
      sock:sendto("hi!", ip, port)
      goto continue
    end
    if who == "closed" then
      print("Closed!!!")
      break
    end
  else
    if who == my_ip and who_port == port then
      io.stdout:write("!")
      goto continue
    end
    io.stdout:write("\n")
    print(msg, who, who_port)
    sock:sendto("hi!", who, who_port)
    recvd = true
  end
  ::continue::
end
