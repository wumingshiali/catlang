# 应用程序图标

## 当前状态

✅ **Windows**: `catc.ico` - 已生成（多尺寸：16, 32, 48, 64, 128, 256）
✅ **Linux**: `catc.png` - 已生成（512x512）
⚠️ **macOS**: 需要在 macOS 系统上生成 `.icns` 文件

## macOS 图标生成

由于当前不在 macOS 系统上，如需生成 macOS 图标：

### 方法 1：在 macOS 上运行转换脚本
```bash
python3 scripts/convert_icon.py
```

### 方法 2：手动生成
```bash
# 1. 准备 iconset
mkdir icon.iconset
sips -z 16 16 icons/catc.png --out icon.iconset/icon_16x16.png
sips -z 32 32 icons/catc.png --out icon.iconset/icon_16x16@2x.png
sips -z 32 32 icons/catc.png --out icon.iconset/icon_32x32.png
sips -z 64 64 icons/catc.png --out icon.iconset/icon_32x32@2x.png
sips -z 128 128 icons/catc.png --out icon.iconset/icon_128x128.png
sips -z 256 256 icons/catc.png --out icon.iconset/icon_128x128@2x.png
sips -z 256 256 icons/catc.png --out icon.iconset/icon_256x256.png
sips -z 512 512 icons/catc.png --out icon.iconset/icon_256x256@2x.png
sips -z 512 512 icons/catc.png --out icon.iconset/icon_512x512.png
sips -z 1024 1024 icons/catc.png --out icon.iconset/icon_512x512@2x.png

# 2. 转换为 icns
iconutil -c icns icon.iconset -o icons/catc.icns

# 3. 清理
rm -rf icon.iconset
```

---

## 重新转换图标

如果需要重新转换，将新图标放在此目录并运行：
```bash
pip install Pillow
python3 scripts/convert_icon.py
```
