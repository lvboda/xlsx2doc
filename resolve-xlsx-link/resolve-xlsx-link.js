const path = require('path');
const xlsx = require('xlsx');

function main() {
  const xlsxPath = path.resolve(process.execPath, '../target.xlsx');
  const xlsxData = xlsx.readFile(xlsxPath);
  const sheetNames = xlsxData.SheetNames;
  const sheet = xlsxData.Sheets[sheetNames[0]];
  let flag = false;
  Object.keys(sheet).forEach((key) => {
    if (sheet[key] && sheet[key]["l"] && sheet[key]["l"]["Target"]) {
      const link = sheet[key]["l"]["Target"];
      if (/^(?:http(s)?:\/\/)?[\w.-]+(?:\.[\w\.-]+)+[\w\-\._~:/?#[\]@!\$&'\*\+,;=.]+$/.test(link)) {
        flag = true;
        sheet[key]["v"] = link;
        sheet[key]["t"] = link;
        sheet[key]["h"] = link;
      }
    }
  });
  if (flag) {
    xlsx.writeFileXLSX(xlsxData, xlsxPath);
  }
};

main();