# Lua String Lit

A tool that will scan a directory for any `.lua` files and collect all string literals they contain.
The output is a map keyed with the file name where the value is a list of objects containing the
literal value and the raw byte offset from the file.

## Example Output

The following is the json output from the following example.

<details>

<summary>example.lua</summary>

```lua

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
```

</details>

```json
{
  "127.0.0.1": [
    "./example/init.lua:10:32",
    "./example/init.lua:12:37"
  ],
  "*": [
    "./example/init.lua:20:26"
  ],
  "socket": [
    "./example/init.lua:1:25"
  ],
  "239.255.255.250": [
    "./example/init.lua:4:13"
  ],
  "ip-multicast-loop": [
    "./example/init.lua:26:24"
  ],
  "0.0.0.0": [
    "./example/init.lua:10:13",
    "./example/init.lua:12:17",
    "./example/init.lua:24:16"
  ],
  "hi!": [
    "./example/init.lua:33:18",
    "./example/init.lua:41:20",
    "./example/init.lua:55:18"
  ],
  "Closed!!!": [
    "./example/init.lua:45:14"
  ],
  "reuseaddr": [
    "./example/init.lua:21:24"
  ],
  "closed": [
    "./example/init.lua:44:16"
  ],
  "!": [
    "./example/init.lua:50:24"
  ],
  "\\n": [
    "./example/init.lua:53:22"
  ],
  "ip-add-membership": [
    "./example/init.lua:22:24"
  ],
  "timeout": [
    "./example/init.lua:38:16"
  ],
  "waiting for message": [
    "./example/init.lua:31:22"
  ],
  ".": [
    "./example/init.lua:39:24"
  ]
}
```
