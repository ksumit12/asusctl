# asusctl
this repo lets you control the rgb keyboard lighting on your asus pc/laptops in ubuntu which is usually controlled by aura or armoury crate in windows
if after entering lsusb in the terminal this is your one of the output then asusctl is perfectly compatiable with your pc/laptop
<code>
Bus 001 Device 002: ID 0b05:1866 ASUSTek Computer, Inc. N-KEY Device
</code>
# Installation

> apt install libclang-dev libudev-dev <br>
> curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh <br>
> make <br>
> udo make install <br>

after installtion it might not work at first then try entering this 
>  systemctl daemon-reload && systemctl restart asusd
