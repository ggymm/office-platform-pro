#!/usr/bin/env zx

console.log('开始更新 Linux Command')
cd('D:/Code/github/linux-command')
await $`git pull`
await $`git checkout gh-pages`
console.log('已经更新 Linux Command')

console.log()

console.log('开始复制 Linux Command 到项目目录')
fs.copy('c/', `${__dirname}/../public/linux-command`, { overwrite: true }, (err) => {
  if (err) {
    console.error(err)
  } else {
    console.log('已经复制 Linux Command 到项目目录')
  }
})
