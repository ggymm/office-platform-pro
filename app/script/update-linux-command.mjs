#!/usr/bin/env zx

const forceUpdate = false

if (forceUpdate) {
  console.log('开始更新 linux command')
  cd('D:/Code/github/linux-command')
  await $`git pull`
  console.log('已经更新 linux command')
  console.log()
} else {
  cd('D:/Code/github/linux-command')
}

console.log('开始检出 master')
await $`git checkout master`
console.log('已经检出 master')
console.log()

console.log('开始复制 linux command data.json 到项目目录')
await fs.copyFile('dist/data.min.json', `${__dirname}/../src/assets/linux/command-data.min.json`)
console.log('已经复制 linux command data.json 到项目目录')
console.log()

console.log('开始检出 gh-pages')
await $`git checkout gh-pages`
console.log('已经检出 gh-pages')
console.log()

console.log('开始复制 linux command html 到项目目录')
fs.copy('html/', `${__dirname}/../public/linux-command`, { overwrite: true }, (err) => {
  if (err) {
    console.error(err)
  } else {
    console.log('已经复制 linux command html 到项目目录')
  }
})
