import { dlopen, FFIType, suffix } from "bun:ffi";

import { platform as processPlatform } from "process";

console.log(processPlatform);

if (processPlatform === "win32") {
  const { symbols: winSymbols } = dlopen(`./foreground_app_win.${suffix}`, {
    get_foreground_app_win: {
      args: [],
      returns: FFIType.cstring,
    },
  });

  console.log(winSymbols.get_foreground_app_win());
}

if (processPlatform === "darwin") {
  const { symbols: macSymbols } = dlopen(`./foreground_app_mac.${suffix}`, {
    get_foreground_app_mac: {
      args: [],
      returns: FFIType.cstring,
    },
  });

  console.log(macSymbols.get_foreground_app_mac());
}

// import ActiveWindow from '@paymoapp/active-window';

// ActiveWindow.initialize();

// if (!ActiveWindow.requestPermissions()) {
// 	console.log('Error: You need to grant screen recording permission in System Preferences > Security & Privacy > Privacy > Screen Recording');
// 	process.exit(0);
// }

// const activeWin = ActiveWindow.getActiveWindow();

// console.log('Window title:', activeWin.title);
// console.log('Application:', activeWin.application);
// console.log('Application path:', activeWin.path);
// console.log('Application PID:', activeWin.pid);
// console.log('Application icon:', activeWin.icon);
