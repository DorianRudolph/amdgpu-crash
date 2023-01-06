This repository documents a reproducible amdgpu crash.

[ORIGINAL README](README.orig.md)

## Crash logs

Full output of `journalctl --boot <id>`

### [crash1.txt](crash1.txt)

<details>
<summary>Relevant(?) excerpt</summary>

```
Jan 06 14:37:38 archdesktop kernel: [drm:gfx_v10_0_priv_reg_irq [amdgpu]] *ERROR* Illegal register access in command stream
Jan 06 14:37:38 archdesktop kernel: [drm:amdgpu_job_timedout [amdgpu]] *ERROR* ring gfx_0.0.0 timeout, signaled seq=516877, emitted seq=516879
Jan 06 14:37:38 archdesktop kernel: [drm:amdgpu_job_timedout [amdgpu]] *ERROR* Process information: process physics pid 37602 thread physics pid 37602
Jan 06 14:37:38 archdesktop kernel: amdgpu 0000:0b:00.0: amdgpu: GPU reset begin!
Jan 06 14:37:38 archdesktop kernel: amdgpu 0000:0b:00.0: amdgpu: free PSP TMR buffer
Jan 06 14:37:38 archdesktop kernel: amdgpu 0000:0b:00.0: amdgpu: MODE1 reset
Jan 06 14:37:38 archdesktop kernel: amdgpu 0000:0b:00.0: amdgpu: GPU mode1 reset
Jan 06 14:37:38 archdesktop kernel: amdgpu 0000:0b:00.0: amdgpu: GPU smu mode1 reset
Jan 06 14:37:39 archdesktop kernel: amdgpu 0000:0b:00.0: amdgpu: GPU reset succeeded, trying to resume
Jan 06 14:37:39 archdesktop kernel: [drm] PCIE GART of 512M enabled (table at 0x0000008000300000).
Jan 06 14:37:39 archdesktop kernel: [drm] VRAM is lost due to GPU reset!
Jan 06 14:37:39 archdesktop kernel: [drm] PSP is resuming...
Jan 06 14:37:39 archdesktop kernel: [drm] reserve 0xa00000 from 0x8001000000 for PSP TMR
Jan 06 14:37:39 archdesktop kernel: amdgpu 0000:0b:00.0: amdgpu: RAS: optional ras ta ucode is not available
Jan 06 14:37:39 archdesktop kernel: amdgpu 0000:0b:00.0: amdgpu: SECUREDISPLAY: securedisplay ta ucode is not available
Jan 06 14:37:39 archdesktop kernel: amdgpu 0000:0b:00.0: amdgpu: SMU is resuming...
Jan 06 14:37:39 archdesktop kernel: amdgpu 0000:0b:00.0: amdgpu: smu driver if version = 0x0000000e, smu fw if version = 0x00000012, smu fw program = 0, version = 0x00413900 (65.57.0)
Jan 06 14:37:39 archdesktop kernel: amdgpu 0000:0b:00.0: amdgpu: SMU driver if version not matched
Jan 06 14:37:39 archdesktop kernel: amdgpu 0000:0b:00.0: amdgpu: use vbios provided pptable
Jan 06 14:37:39 archdesktop kernel: amdgpu 0000:0b:00.0: amdgpu: SMU is resumed successfully!
Jan 06 14:37:39 archdesktop kernel: [drm] DMUB hardware initialized: version=0x02020017
Jan 06 14:37:39 archdesktop /usr/lib/gdm-wayland-session[4553]: amdgpu: The CS has been rejected (-125), but the context isn't robust.
Jan 06 14:37:39 archdesktop /usr/lib/gdm-wayland-session[4553]: amdgpu: The process will be terminated.
Jan 06 14:37:39 archdesktop kernel: [drm] kiq ring mec 2 pipe 1 q 0
Jan 06 14:37:39 archdesktop kernel: [drm] VCN decode and encode initialized successfully(under DPG Mode).
Jan 06 14:37:39 archdesktop kernel: [drm] JPEG decode initialized successfully.
Jan 06 14:37:39 archdesktop kernel: amdgpu 0000:0b:00.0: amdgpu: ring gfx_0.0.0 uses VM inv eng 0 on hub 0
Jan 06 14:37:39 archdesktop kernel: amdgpu 0000:0b:00.0: amdgpu: ring comp_1.0.0 uses VM inv eng 1 on hub 0
Jan 06 14:37:39 archdesktop kernel: amdgpu 0000:0b:00.0: amdgpu: ring comp_1.1.0 uses VM inv eng 4 on hub 0
Jan 06 14:37:39 archdesktop kernel: amdgpu 0000:0b:00.0: amdgpu: ring comp_1.2.0 uses VM inv eng 5 on hub 0
Jan 06 14:37:39 archdesktop kernel: amdgpu 0000:0b:00.0: amdgpu: ring comp_1.3.0 uses VM inv eng 6 on hub 0
Jan 06 14:37:39 archdesktop kernel: amdgpu 0000:0b:00.0: amdgpu: ring comp_1.0.1 uses VM inv eng 7 on hub 0
Jan 06 14:37:39 archdesktop kernel: amdgpu 0000:0b:00.0: amdgpu: ring comp_1.1.1 uses VM inv eng 8 on hub 0
Jan 06 14:37:39 archdesktop kernel: amdgpu 0000:0b:00.0: amdgpu: ring comp_1.2.1 uses VM inv eng 9 on hub 0
Jan 06 14:37:39 archdesktop kernel: amdgpu 0000:0b:00.0: amdgpu: ring comp_1.3.1 uses VM inv eng 10 on hub 0
Jan 06 14:37:39 archdesktop kernel: amdgpu 0000:0b:00.0: amdgpu: ring kiq_2.1.0 uses VM inv eng 11 on hub 0
Jan 06 14:37:39 archdesktop kernel: amdgpu 0000:0b:00.0: amdgpu: ring sdma0 uses VM inv eng 12 on hub 0
Jan 06 14:37:39 archdesktop kernel: amdgpu 0000:0b:00.0: amdgpu: ring sdma1 uses VM inv eng 13 on hub 0
Jan 06 14:37:39 archdesktop kernel: amdgpu 0000:0b:00.0: amdgpu: ring vcn_dec_0 uses VM inv eng 0 on hub 1
Jan 06 14:37:39 archdesktop kernel: amdgpu 0000:0b:00.0: amdgpu: ring vcn_enc_0.0 uses VM inv eng 1 on hub 1
Jan 06 14:37:39 archdesktop kernel: amdgpu 0000:0b:00.0: amdgpu: ring vcn_enc_0.1 uses VM inv eng 4 on hub 1
Jan 06 14:37:39 archdesktop kernel: amdgpu 0000:0b:00.0: amdgpu: ring jpeg_dec uses VM inv eng 5 on hub 1
Jan 06 14:37:39 archdesktop kernel: amdgpu 0000:0b:00.0: amdgpu: recover vram bo from shadow start
Jan 06 14:37:39 archdesktop kernel: amdgpu 0000:0b:00.0: amdgpu: recover vram bo from shadow done
Jan 06 14:37:39 archdesktop kernel: [drm] Skip scheduling IBs!
Jan 06 14:37:39 archdesktop kernel: [drm] Skip scheduling IBs!
Jan 06 14:37:39 archdesktop kernel: amdgpu 0000:0b:00.0: amdgpu: GPU reset(2) succeeded!
Jan 06 14:37:39 archdesktop kernel: [drm] Skip scheduling IBs!
// [...] above message repeats many timers
Jan 06 14:37:39 archdesktop kernel: [drm:amdgpu_cs_ioctl [amdgpu]] *ERROR* Failed to initialize parser -125!
Jan 06 14:37:39 archdesktop kernel: [drm:amdgpu_cs_ioctl [amdgpu]] *ERROR* Failed to initialize parser -125!
Jan 06 14:37:39 archdesktop chromium[10672]: Error reading events from display: Broken pipe
Jan 06 14:37:39 archdesktop /usr/lib/gdm-wayland-session[12295]: (EE) failed to read Wayland events: Broken pipe
Jan 06 14:37:39 archdesktop konsole[5460]: The Wayland connection broke. Did the Wayland compositor die?
Jan 06 14:37:39 archdesktop nm-applet[4723]: Error reading events from display: Broken pipe
Jan 06 14:37:39 archdesktop polkit-gnome-au[4727]: Error reading events from display: Broken pipe
Jan 06 14:37:39 archdesktop waybar[4717]: Error reading events from display: Broken pipe
Jan 06 14:37:39 archdesktop polkitd[2839]: Unregistered Authentication Agent for unix-session:3 (system bus name :1.61, object path /org/gnome/PolicyKit1/AuthenticationAgent, locale en_US.UTF-8) (disconnected from bus)
Jan 06 14:37:39 archdesktop gdm-wayland-session[4547]: GLib: Source ID 2 was not found when attempting to remove it
Jan 06 14:37:39 archdesktop gdm-password][4493]: pam_unix(gdm-password:session): session closed for user dorian
Jan 06 14:37:39 archdesktop discord.desktop[12405]: X connection to :0 broken (explicit kill or server shutdown).
Jan 06 14:37:39 archdesktop kernel: [drm:dc_add_plane_to_context [amdgpu]] *ERROR* Head pipe not found for stream_state 00000000b4ea039a !
Jan 06 14:37:39 archdesktop kernel: [drm:dc_add_plane_to_context [amdgpu]] *ERROR* Head pipe not found for stream_state 00000000b4ea039a !
Jan 06 14:37:39 archdesktop kernel: [drm:dc_add_plane_to_context [amdgpu]] *ERROR* Head pipe not found for stream_state 00000000b4ea039a !
Jan 06 14:37:39 archdesktop audit[4493]: USER_END pid=4493 uid=0 auid=1000 ses=3 msg='op=PAM:session_close grantors=pam_loginuid,pam_keyinit,pam_systemd_home,pam_limits,pam_unix,pam_permit,pam_mail,pam_systemd,pam_env,pam_gnome_keyring acct="dorian" exe="/usr/lib/gdm-session-worker" hostname=archdesktop addr=? terminal=/dev/tty2 res=success'
Jan 06 14:37:39 archdesktop audit[4493]: CRED_DISP pid=4493 uid=0 auid=1000 ses=3 msg='op=PAM:setcred grantors=pam_shells,pam_faillock,pam_permit,pam_faillock,pam_gnome_keyring acct="dorian" exe="/usr/lib/gdm-session-worker" hostname=archdesktop addr=? terminal=/dev/tty2 res=success'
Jan 06 14:37:39 archdesktop kernel: audit: type=1106 audit(1673012259.945:711): pid=4493 uid=0 auid=1000 ses=3 msg='op=PAM:session_close grantors=pam_loginuid,pam_keyinit,pam_systemd_home,pam_limits,pam_unix,pam_permit,pam_mail,pam_systemd,pam_env,pam_gnome_keyring acct="dorian" exe="/usr/lib/gdm-session-worker" hostname=archdesktop addr=? terminal=/dev/tty2 res=success'
Jan 06 14:37:39 archdesktop kernel: audit: type=1104 audit(1673012259.945:712): pid=4493 uid=0 auid=1000 ses=3 msg='op=PAM:setcred grantors=pam_shells,pam_faillock,pam_permit,pam_faillock,pam_gnome_keyring acct="dorian" exe="/usr/lib/gdm-session-worker" hostname=archdesktop addr=? terminal=/dev/tty2 res=success'
Jan 06 14:37:39 archdesktop kernel: [drm:dc_remove_stream_from_ctx [amdgpu]] *ERROR* Pipe not found for stream 00000000b4ea039a !
Jan 06 14:37:39 archdesktop kernel: ------------[ cut here ]------------
Jan 06 14:37:39 archdesktop kernel: atomic remove_fb failed with -22
Jan 06 14:37:39 archdesktop kernel: WARNING: CPU: 15 PID: 263 at drivers/gpu/drm/drm_framebuffer.c:1130 drm_framebuffer_remove+0x3b5/0x660
Jan 06 14:37:39 archdesktop kernel: Modules linked in: tun iptable_filter cfg80211 rfcomm snd_seq_dummy snd_hrtimer snd_seq veth xt_nat xt_conntrack xt_MASQUERADE nf_conntrack_netlink xt_addrtype br_netfilter nft_masq bridge stp llc nft_chain_nat nf_nat nf_conntrack nf_defrag_ipv6 nf_defrag_ipv4 ip6t_REJECT nf_reject_ipv6 ipt_REJECT nf_reject_ipv4 xt_multiport xt_cgroup xt_mark xt_owner xt_tcpudp nft_compat nf_tables nfnetlink cmac algif_hash algif_skcipher af_alg bnep ext4 eeepc_wmi snd_usb_audio asus_wmi snd_usbmidi_lib mbcache sparse_keymap snd_rawmidi jbd2 intel_rapl_msr wmi_bmof platform_profile mxm_wmi snd_seq_device snd_hda_codec_realtek mc snd_hda_codec_generic intel_rapl_common amdgpu ledtrig_audio snd_hda_codec_hdmi btusb snd_hda_intel btrtl snd_intel_dspcfg btbcm snd_intel_sdw_acpi btintel snd_hda_codec btmtk vfat gpu_sched edac_mce_amd mousedev snd_hda_core fat drm_buddy bluetooth joydev kvm_amd video snd_hwdep ecdh_generic snd_pcm drm_ttm_helper asus_wmi_sensors rfkill snd_timer asus_ec_sensors
Jan 06 14:37:39 archdesktop kernel:  crc16 kvm ttm lzo_rle drm_display_helper irqbypass snd sp5100_tco igb rapl pcspkr cec k10temp i2c_piix4 soundcore dca wmi gpio_amdpt gpio_generic acpi_cpufreq mac_hid vboxnetflt(OE) vboxnetadp(OE) vboxdrv(OE) dm_multipath sg crypto_user fuse zram ip_tables x_tables btrfs blake2b_generic libcrc32c crc32c_generic xor raid6_pq dm_crypt cbc encrypted_keys trusted asn1_encoder tee hid_logitech_hidpp hid_logitech_dj hid_uclogic usbhid dm_mod crct10dif_pclmul crc32_pclmul crc32c_intel polyval_clmulni polyval_generic gf128mul ghash_clmulni_intel sha512_ssse3 nvme aesni_intel crypto_simd nvme_core ccp cryptd xhci_pci nvme_common xhci_pci_renesas
Jan 06 14:37:39 archdesktop kernel: CPU: 15 PID: 263 Comm: kworker/15:1 Tainted: G           OE      6.1.2-zen1-1-zen #1 ebd30fdca46506e3691fb11aed133ac3be9bd236
Jan 06 14:37:39 archdesktop kernel: Hardware name: System manufacturer System Product Name/PRIME X470-PRO, BIOS 6024 02/25/2022
Jan 06 14:37:39 archdesktop kernel: Workqueue: events drm_mode_rmfb_work_fn
Jan 06 14:37:39 archdesktop kernel: RIP: 0010:drm_framebuffer_remove+0x3b5/0x660
Jan 06 14:37:39 archdesktop kernel: Code: fd ff ff 48 8d 7c 24 18 e8 d8 a2 ff ff 48 8d 7c 24 18 e8 9e a1 ff ff ba f4 ff ff ff 89 d6 48 c7 c7 08 2a b2 b3 e8 13 65 40 00 <0f> 0b e9 94 fe ff ff be 03 00 00 00 48 89 df e8 77 d4 ca ff e9 57
Jan 06 14:37:39 archdesktop kernel: RSP: 0018:ffffbe8d00bb7dc8 EFLAGS: 00010282
Jan 06 14:37:39 archdesktop kernel: RAX: 0000000000000000 RBX: ffff97ce20cdae00 RCX: 0000000000000027
Jan 06 14:37:39 archdesktop kernel: RDX: ffff97daaede1668 RSI: 0000000000000001 RDI: ffff97daaede1660
Jan 06 14:37:39 archdesktop kernel: RBP: ffff97cbc5e76d00 R08: 0000000000000001 R09: 00000000ffffffea
Jan 06 14:37:39 archdesktop kernel: R10: ffff97daae950000 R11: 00000000ffffe000 R12: ffff97cc0ffe02d0
Jan 06 14:37:39 archdesktop kernel: R13: 0000000000000020 R14: 00000000ffffffea R15: 0000000000000000
Jan 06 14:37:39 archdesktop kernel: FS:  0000000000000000(0000) GS:ffff97daaedc0000(0000) knlGS:0000000000000000
Jan 06 14:37:39 archdesktop kernel: CS:  0010 DS: 0000 ES: 0000 CR0: 0000000080050033
Jan 06 14:37:39 archdesktop kernel: CR2: 000033526ba59ea8 CR3: 0000000106e56000 CR4: 0000000000750ee0
Jan 06 14:37:39 archdesktop kernel: PKRU: 55555554
Jan 06 14:37:39 archdesktop kernel: Call Trace:
Jan 06 14:37:39 archdesktop kernel:  <TASK>
Jan 06 14:37:39 archdesktop kernel:  drm_mode_rmfb_work_fn+0x55/0xa0
Jan 06 14:37:39 archdesktop kernel:  process_one_work+0x258/0x410
Jan 06 14:37:39 archdesktop kernel:  worker_thread+0x244/0x4d0
Jan 06 14:37:39 archdesktop kernel:  ? process_one_work+0x410/0x410
Jan 06 14:37:39 archdesktop kernel:  kthread+0xde/0x110
Jan 06 14:37:39 archdesktop kernel:  ? kthread_complete_and_exit+0x20/0x20
Jan 06 14:37:39 archdesktop kernel:  ret_from_fork+0x22/0x30
Jan 06 14:37:39 archdesktop kernel:  </TASK>
Jan 06 14:37:39 archdesktop kernel: ---[ end trace 0000000000000000 ]---
Jan 06 14:37:39 archdesktop kernel: [drm:dc_remove_stream_from_ctx [amdgpu]] *ERROR* Pipe not found for stream 000000002f3f3ba3 !
Jan 06 14:37:39 archdesktop kernel: ------------[ cut here ]------------
Jan 06 14:37:39 archdesktop kernel: atomic remove_fb failed with -22
Jan 06 14:37:39 archdesktop kernel: WARNING: CPU: 15 PID: 263 at drivers/gpu/drm/drm_framebuffer.c:1130 drm_framebuffer_remove+0x3b5/0x660
Jan 06 14:37:39 archdesktop kernel: Modules linked in: tun iptable_filter cfg80211 rfcomm snd_seq_dummy snd_hrtimer snd_seq veth xt_nat xt_conntrack xt_MASQUERADE nf_conntrack_netlink xt_addrtype br_netfilter nft_masq bridge stp llc nft_chain_nat nf_nat nf_conntrack nf_defrag_ipv6 nf_defrag_ipv4 ip6t_REJECT nf_reject_ipv6 ipt_REJECT nf_reject_ipv4 xt_multiport xt_cgroup xt_mark xt_owner xt_tcpudp nft_compat nf_tables nfnetlink cmac algif_hash algif_skcipher af_alg bnep ext4 eeepc_wmi snd_usb_audio asus_wmi snd_usbmidi_lib mbcache sparse_keymap snd_rawmidi jbd2 intel_rapl_msr wmi_bmof platform_profile mxm_wmi snd_seq_device snd_hda_codec_realtek mc snd_hda_codec_generic intel_rapl_common amdgpu ledtrig_audio snd_hda_codec_hdmi btusb snd_hda_intel btrtl snd_intel_dspcfg btbcm snd_intel_sdw_acpi btintel snd_hda_codec btmtk vfat gpu_sched edac_mce_amd mousedev snd_hda_core fat drm_buddy bluetooth joydev kvm_amd video snd_hwdep ecdh_generic snd_pcm drm_ttm_helper asus_wmi_sensors rfkill snd_timer asus_ec_sensors
Jan 06 14:37:39 archdesktop kernel:  crc16 kvm ttm lzo_rle drm_display_helper irqbypass snd sp5100_tco igb rapl pcspkr cec k10temp i2c_piix4 soundcore dca wmi gpio_amdpt gpio_generic acpi_cpufreq mac_hid vboxnetflt(OE) vboxnetadp(OE) vboxdrv(OE) dm_multipath sg crypto_user fuse zram ip_tables x_tables btrfs blake2b_generic libcrc32c crc32c_generic xor raid6_pq dm_crypt cbc encrypted_keys trusted asn1_encoder tee hid_logitech_hidpp hid_logitech_dj hid_uclogic usbhid dm_mod crct10dif_pclmul crc32_pclmul crc32c_intel polyval_clmulni polyval_generic gf128mul ghash_clmulni_intel sha512_ssse3 nvme aesni_intel crypto_simd nvme_core ccp cryptd xhci_pci nvme_common xhci_pci_renesas
Jan 06 14:37:39 archdesktop kernel: CPU: 15 PID: 263 Comm: kworker/15:1 Tainted: G        W  OE      6.1.2-zen1-1-zen #1 ebd30fdca46506e3691fb11aed133ac3be9bd236
Jan 06 14:37:39 archdesktop kernel: Hardware name: System manufacturer System Product Name/PRIME X470-PRO, BIOS 6024 02/25/2022
Jan 06 14:37:39 archdesktop kernel: Workqueue: events drm_mode_rmfb_work_fn
Jan 06 14:37:39 archdesktop kernel: RIP: 0010:drm_framebuffer_remove+0x3b5/0x660
Jan 06 14:37:39 archdesktop kernel: Code: fd ff ff 48 8d 7c 24 18 e8 d8 a2 ff ff 48 8d 7c 24 18 e8 9e a1 ff ff ba f4 ff ff ff 89 d6 48 c7 c7 08 2a b2 b3 e8 13 65 40 00 <0f> 0b e9 94 fe ff ff be 03 00 00 00 48 89 df e8 77 d4 ca ff e9 57
Jan 06 14:37:39 archdesktop kernel: RSP: 0018:ffffbe8d00bb7dc8 EFLAGS: 00010282
Jan 06 14:37:39 archdesktop kernel: RAX: 0000000000000000 RBX: ffff97ce20cdae00 RCX: 0000000000000027
Jan 06 14:37:39 archdesktop kernel: RDX: ffff97daaede1668 RSI: 0000000000000001 RDI: ffff97daaede1660
Jan 06 14:37:39 archdesktop kernel: RBP: ffff97cc2cbd6100 R08: 0000000000000001 R09: 00000000ffffffea
Jan 06 14:37:39 archdesktop kernel: R10: ffff97daae950000 R11: 00000000ffffe000 R12: ffff97cc0ffe02d0
Jan 06 14:37:39 archdesktop kernel: R13: 0000000000000010 R14: 00000000ffffffea R15: 0000000000000000
Jan 06 14:37:39 archdesktop kernel: FS:  0000000000000000(0000) GS:ffff97daaedc0000(0000) knlGS:0000000000000000
Jan 06 14:37:39 archdesktop kernel: CS:  0010 DS: 0000 ES: 0000 CR0: 0000000080050033
Jan 06 14:37:39 archdesktop kernel: CR2: 000033526ba59ea8 CR3: 0000000106e56000 CR4: 0000000000750ee0
Jan 06 14:37:39 archdesktop kernel: PKRU: 55555554
Jan 06 14:37:39 archdesktop kernel: Call Trace:
Jan 06 14:37:39 archdesktop kernel:  <TASK>
Jan 06 14:37:39 archdesktop kernel:  drm_mode_rmfb_work_fn+0x55/0xa0
Jan 06 14:37:39 archdesktop kernel:  process_one_work+0x258/0x410
Jan 06 14:37:39 archdesktop kernel:  worker_thread+0x244/0x4d0
Jan 06 14:37:39 archdesktop kernel:  ? process_one_work+0x410/0x410
Jan 06 14:37:39 archdesktop kernel:  kthread+0xde/0x110
Jan 06 14:37:39 archdesktop kernel:  ? kthread_complete_and_exit+0x20/0x20
Jan 06 14:37:39 archdesktop kernel:  ret_from_fork+0x22/0x30
Jan 06 14:37:39 archdesktop kernel:  </TASK>
Jan 06 14:37:39 archdesktop kernel: ---[ end trace 0000000000000000 ]---
Jan 06 14:37:39 archdesktop kernel: [drm:dc_add_plane_to_context [amdgpu]] *ERROR* Head pipe not found for stream_state 00000000b4ea039a !
Jan 06 14:37:39 archdesktop kernel: [drm:dc_add_plane_to_context [amdgpu]] *ERROR* Head pipe not found for stream_state 00000000b4ea039a !
Jan 06 14:37:40 archdesktop kernel: [drm:dc_add_plane_to_context [amdgpu]] *ERROR* Head pipe not found for stream_state 00000000b4ea039a !
``` 
</details>

### [crash2.txt](crash2.txt)

The second log does not have the "cut here" part. It ends with

```
Jan 06 14:52:49 archdesktop kernel: [drm:gfx_v10_0_priv_reg_irq [amdgpu]] *ERROR* Illegal register access in command stream
Jan 06 14:52:49 archdesktop kernel: [drm:amdgpu_job_timedout [amdgpu]] *ERROR* ring gfx_0.0.0 timeout, signaled seq=502030, emitted seq=502032
Jan 06 14:52:49 archdesktop kernel: [drm:amdgpu_job_timedout [amdgpu]] *ERROR* Process information: process physics pid 35368 thread physics pid 35368
Jan 06 14:52:49 archdesktop kernel: amdgpu 0000:0b:00.0: amdgpu: GPU reset begin!
Jan 06 14:52:50 archdesktop kernel: amdgpu 0000:0b:00.0: amdgpu: free PSP TMR buffer
Jan 06 14:52:50 archdesktop /usr/lib/gdm-wayland-session[4904]: amdgpu: The CS has been rejected (-125), but the context isn't robust.
Jan 06 14:52:50 archdesktop /usr/lib/gdm-wayland-session[4904]: amdgpu: The process will be terminated.
Jan 06 14:52:51 archdesktop konsole[13430]: The Wayland connection broke. Did the Wayland compositor die?
Jan 06 14:52:51 archdesktop konsole[7984]: The Wayland connection broke. Did the Wayland compositor die?
Jan 06 14:52:51 archdesktop chromium[11311]: Error reading events from display: Broken pipe
```
