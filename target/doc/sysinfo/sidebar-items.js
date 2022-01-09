initSidebarItems({"enum":[["DiskType","Enum containing the different supported disks types."],["ProcessStatus","Enum describing the different status of a process."],["Signal","An enum representing signals on UNIX-like systems."]],"fn":[["get_current_pid","Returns the pid for the current process."],["set_open_files_limit","This function is only used on linux targets, on the other platforms it does nothing and returns `false`."]],"struct":[["Component","Struct containing a component information (temperature and name for the moment)."],["Disk","Struct containing a disk information."],["DiskUsage","Type containing read and written bytes."],["Gid","A group id wrapping a platform specific type"],["LoadAvg","A struct representing system load average value."],["NetworkData","Contains network interface information."],["Networks","Networks interfaces."],["NetworksIter","Iterator over network interfaces."],["Process","Struct containing information of a process."],["ProcessRefreshKind","Used to determine what you want to refresh specifically on the `Process` type."],["Processor","Struct containing information of a processor."],["RefreshKind","Used to determine what you want to refresh specifically on the `System` type."],["System","Structs containing system’s information."],["Uid","A user id wrapping a platform specific type"],["User","Type containing user information."]],"trait":[["AsU32","Trait to have a common fallback for the [`Pid`][crate::Pid] type."],["ComponentExt","Getting a component temperature information."],["DiskExt","Contains all the methods of the [`Disk`][crate::Disk] struct."],["NetworkExt","Getting volume of received and transmitted data."],["NetworksExt","Interacting with network interfaces."],["ProcessExt","Contains all the methods of the [`Process`][crate::Process] struct."],["ProcessorExt","Contains all the methods of the [`Processor`][crate::Processor] struct."],["SystemExt","Contains all the methods of the [`System`][crate::System] type."],["UserExt","Getting information for a user."]],"type":[["Pid","Process id."]]});