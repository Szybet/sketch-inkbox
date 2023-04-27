#!/bin/bash -x

cp latest-binary-sketch inkbox_userapp/sketch/app-bin/sketch.bin

# Very important
rm -f inkbox_userapp/sketch.isa.dgst
rm -f inkbox_userapp/sketch.isa

mksquashfs inkbox_userapp/sketch/* inkbox_userapp/sketch.isa

# Yes, here are my private keys. Is providing this info a security thread? no.
openssl dgst -sha256 -sign /home/szybet/inkbox-keys/userapps.pem -out inkbox_userapp/sketch.isa.dgst inkbox_userapp/sketch.isa

servername="root@10.42.0.28"
passwd="root"

sshpass -p $passwd ssh $servername "bash -c \"ifsctl mnt rootfs rw\""
sshpass -p $passwd ssh $servername "bash -c \"mkdir /data/onboard/.apps/\""
sshpass -p $passwd ssh $servername "bash -c \"mkdir /data/onboard/.apps/sketch\""
sshpass -p $passwd ssh $servername "bash -c \"rm /data/onboard/.apps/sketch/sketch.isa\""
sshpass -p $passwd ssh $servername "bash -c \"rm /data/onboard/.apps/sketch/sketch.isa.dgst\""
sshpass -p $passwd ssh $servername "bash -c \"rm /data/onboard/.apps/sketch/app.json\""

sshpass -p $passwd scp inkbox_userapp/app.json $servername:/data/onboard/.apps/sketch/
sshpass -p $passwd scp inkbox_userapp/sketch.isa.dgst $servername:/data/onboard/.apps/sketch/
sshpass -p $passwd scp inkbox_userapp/sketch.isa $servername:/data/onboard/.apps/sketch/


sshpass -p $passwd ssh $servername "bash -c \"sync\""

sshpass -p $passwd ssh $servername "bash -c \"killall -9 sketch.sh\"" || EXIT_CODE=0

sshpass -p $passwd ssh $servername "bash -c \"service gui_apps restart\""

# sshpass -p $passwd ssh $servername "bash -c \"service inkbox_gui restart\"" & # to get logs

# To update main json
# sleep 15
# sshpass -p $passwd ssh $servername "bash -c \"touch /kobo/tmp/rescan_userapps\"" # This gets deleted by service restart
# sshpass -p $passwd ssh $servername "bash -c \"killall inkbox-bin\""

# sshpass -p $passwd ssh $servername "bash -c \"rc-service gui_apps restart\""


