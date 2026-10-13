# Windows single-instance IPC

NexoPDF hands files from a later launch to the existing window over an AF_UNIX socket.
The socket and election lock live in `%USERPROFILE%\.nexopdf-ipc`, with a separate
socket name per Windows sign-in and a different prefix for portable copies.
Settings, logs and crash recovery keep their existing locations.

## Why the socket is outside AppData

On the verified Windows 11 build 26300 environment, sockets in AppData and Temp
bind successfully but connections return Winsock 10022. Both `uds_windows` and
the independent .NET socket implementation reproduce it there. The same original
Rust dependency connects successfully outside AppData, including the private
profile IPC directory. This is a storage-location problem, not a broken socket
address size or a reason to change the transport.

The fix therefore moves only the IPC files into the private user profile. It does
not modify the dependency, introduce a TCP listener, or enable UI control or MCP.
Windows ACL inspection of the new directory grants access to the owner, SYSTEM
and Administrators; it does not grant other ordinary accounts access.

## Regression coverage

The original exchange, simultaneous-election and stale-socket tests now exercise
the actual production IPC directory, with isolated test subdirectories. They do
not skip failures or mock connections. A new test hands Unicode file paths over
a socket whose own filename is Unicode. All nine single-instance tests pass.

The native application was also verified with isolated portable data and two
synthetic PDFs: the second process exited with success and UI inspection of the
first window showed both documents, including an accented/Japanese filename.

The existing safe fallback remains: a failed hand-off opens a separate window,
and options such as `--new-window` keep their existing behaviour.
