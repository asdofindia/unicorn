var searchIndex = {};
searchIndex["unicorn"] = {"doc":"","items":[[5,"get_version","unicorn","Return version as a formatted string in semver format",null,{"inputs":[],"output":{"name":"string"}}],[0,"logger","","Logger module for unicorn",null,null],[3,"CLILogger","unicorn::logger","Logs to console",null,null],[11,"init","","",0,{"inputs":[{"name":"str"}],"output":{"name":"result"}}],[11,"enabled","","",0,{"inputs":[{"name":"clilogger"},{"name":"logmetadata"}],"output":{"name":"bool"}}],[11,"log","","",0,{"inputs":[{"name":"clilogger"},{"name":"logrecord"}],"output":null}],[0,"core","unicorn","",null,null],[0,"processor","unicorn::core","Processes incoming messages to core and routes them to appropriate handlers",null,null],[3,"ProcessMsg","unicorn::core::processor","",null,null],[11,"process","","Procedure for processing incoming messages",1,{"inputs":[{"name":"processmsg"},{"name":"string"}],"output":{"name":"vec"}}],[0,"server","unicorn::core","",null,null],[5,"run","unicorn::core::server","Run the core service",null,{"inputs":[],"output":null}],[0,"gateway","unicorn","",null,null],[0,"client","unicorn::gateway","",null,null],[5,"run","unicorn::gateway::client","Run the gateway service",null,{"inputs":[],"output":null}],[0,"messages","unicorn","Module for stating message contracts and decoding/encoding messages",null,null],[4,"Msg","unicorn::messages","Message structure definitions. Each variant of this enum forms a type of message.",null,null],[13,"Status","","Status message. Used to send status of different components",2,null],[12,"id","unicorn::messages::Msg","",2,null],[12,"state","","",2,null],[12,"msg","","",2,null],[13,"Heartbeat","unicorn::messages","Heartbeat message. Used to check liveliness of connected peers.",2,null],[12,"id","unicorn::messages::Msg","",2,null],[12,"count","","",2,null],[13,"Error","unicorn::messages","Error message containing a description",2,null],[13,"Ok","","Ok message. Sent to indicate success but nothing useful to return",2,null],[5,"encode","","Generic encoder for all messages. Encodes message structures to JSON strings",null,{"inputs":[{"name":"msg"}],"output":{"name":"option"}}],[5,"encode_bytes","","Convenience method to message structures directly to serialized bytes",null,{"inputs":[{"name":"msg"}],"output":{"name":"vec"}}],[5,"decode","","Generic decoder for all messages. Decodes JSON strings to message structures",null,{"inputs":[{"name":"string"}],"output":{"name":"option"}}],[0,"common","","Common message structures shared by more than one components",null,null],[3,"ID","unicorn::messages::common","Identity message. Used by every message that needs an ID",null,null],[12,"uuid","","",3,null],[12,"component","","",3,null],[4,"Components","","List of main component types in unicorn",null,null],[13,"Core","","",4,null],[13,"Gateway","","",4,null],[13,"DataStore","","",4,null],[13,"DataElement","","",4,null],[13,"DataHandler","","",4,null],[13,"ClientAdapter","","",4,null],[4,"State","","Different states in which a component can be at any given time",null,null],[13,"CONNECTING","","",5,null],[13,"READY","","",5,null],[13,"BUSY","","",5,null],[13,"RETRYING","","",5,null],[13,"SUCCESS","","",5,null],[13,"FAILED","","",5,null],[13,"OFFLINE","","",5,null],[11,"eq","","",4,{"inputs":[{"name":"components"},{"name":"components"}],"output":{"name":"bool"}}],[11,"ne","","",4,{"inputs":[{"name":"components"},{"name":"components"}],"output":{"name":"bool"}}],[11,"fmt","","",4,{"inputs":[{"name":"components"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"decode","","",4,{"inputs":[{"name":"__d"}],"output":{"name":"result"}}],[11,"encode","","",4,{"inputs":[{"name":"components"},{"name":"__s"}],"output":{"name":"result"}}],[11,"eq","","",5,{"inputs":[{"name":"state"},{"name":"state"}],"output":{"name":"bool"}}],[11,"ne","","",5,{"inputs":[{"name":"state"},{"name":"state"}],"output":{"name":"bool"}}],[11,"fmt","","",5,{"inputs":[{"name":"state"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"decode","","",5,{"inputs":[{"name":"__d"}],"output":{"name":"result"}}],[11,"encode","","",5,{"inputs":[{"name":"state"},{"name":"__s"}],"output":{"name":"result"}}],[11,"eq","","",3,{"inputs":[{"name":"id"},{"name":"id"}],"output":{"name":"bool"}}],[11,"ne","","",3,{"inputs":[{"name":"id"},{"name":"id"}],"output":{"name":"bool"}}],[11,"fmt","","",3,{"inputs":[{"name":"id"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"decode","","",3,{"inputs":[{"name":"__d"}],"output":{"name":"result"}}],[11,"encode","","",3,{"inputs":[{"name":"id"},{"name":"__s"}],"output":{"name":"result"}}],[11,"eq","unicorn::messages","",2,{"inputs":[{"name":"msg"},{"name":"msg"}],"output":{"name":"bool"}}],[11,"ne","","",2,{"inputs":[{"name":"msg"},{"name":"msg"}],"output":{"name":"bool"}}],[11,"fmt","","",2,{"inputs":[{"name":"msg"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"decode","","",2,{"inputs":[{"name":"__d"}],"output":{"name":"result"}}],[11,"encode","","",2,{"inputs":[{"name":"msg"},{"name":"__s"}],"output":{"name":"result"}}],[0,"network","unicorn","Network abstraction layer that handles low level network topology\nand exposes a simple higher level API",null,null],[3,"Net","unicorn::network","Network structure that holds network topology information. Aka,\nthe network interface.",null,null],[3,"Stream","","Structure that holds TcpStream and provides convenience methods",null,null],[4,"Status","","List of states for a network connection",null,null],[13,"DISCONNECTED","","",6,null],[13,"CONNECTING","","",6,null],[13,"READY","","",6,null],[13,"RECONNECTING","","",6,null],[13,"TIMEOUT","","",6,null],[13,"ERROR","","",6,null],[5,"bind","","Convenience function to bind a new Net",null,{"inputs":[{"name":"string"}],"output":{"name":"result"}}],[5,"connect","","Convenience function to connect to an existing Net and return a Stream",null,{"inputs":[{"name":"string"}],"output":{"name":"result"}}],[11,"addr","","Returns the addr",7,{"inputs":[{"name":"net"}],"output":{"name":"string"}}],[11,"status","","Returns the status",7,{"inputs":[{"name":"net"}],"output":{"name":"status"}}],[11,"num_workers","","Set number of workers for processing incoming streams in parallel",7,{"inputs":[{"name":"net"},{"name":"usize"}],"output":null}],[11,"bind","","Start a listener on an existing network interface",7,{"inputs":[{"name":"string"}],"output":{"name":"result"}}],[11,"recv","","Process incoming TCP stream",7,{"inputs":[{"name":"net"},{"name":"processor"}],"output":null}],[11,"new","","Create a new stream",8,{"inputs":[{"name":"tcpstream"}],"output":{"name":"stream"}}],[11,"connect","","Connect to a TCP listener socket and get back a stream",8,{"inputs":[{"name":"string"},{"name":"bool"}],"output":{"name":"result"}}],[11,"recv","","Process a given TcpStream and invoke processor",8,{"inputs":[{"name":"stream"}],"output":{"name":"option"}}],[11,"send","","Procedure for sending messages through a specified stream",8,{"inputs":[{"name":"stream"},{"name":"vec"}],"output":null}],[11,"flush","","Call flush on the buffer to send out currently stored messages\nimmediately",8,{"inputs":[{"name":"stream"}],"output":{"name":"result"}}],[8,"Processor","","Provides an interface for creating processor types",null,null],[10,"process","","",9,{"inputs":[{"name":"processor"},{"name":"string"}],"output":{"name":"vec"}}],[11,"clone","","",6,{"inputs":[{"name":"status"}],"output":{"name":"status"}}],[17,"VERSION","unicorn","unicorn version",null,null]],"paths":[[3,"CLILogger"],[3,"ProcessMsg"],[4,"Msg"],[3,"ID"],[4,"Components"],[4,"State"],[4,"Status"],[3,"Net"],[3,"Stream"],[8,"Processor"]]};
initSearch(searchIndex);
