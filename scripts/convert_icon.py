#!/usr/bin/env python3
"""
图标转换脚本
将用户上传的图标转换为 Windows (.ico) 和 macOS (.icns) 格式

使用方法:
1. 将原始图标图片放在 icons/ 目录，命名为 source.png 或 source.jpg
2. 运行：python3 scripts/convert_icon.py
3. 转换后的图标将保存在 icons/ 目录
"""

import os
import sys
import subprocess
from pathlib import Path

# 项目根目录
PROJECT_ROOT = Path(__file__).parent.parent
ICONS_DIR = PROJECT_ROOT / "icons"

def check_dependencies():
    """检查必要的依赖"""
    try:
        from PIL import Image
        return True
    except ImportError:
        print("❌ 缺少 Pillow 库，请安装：pip install Pillow")
        return False

def convert_to_ico(source_path: Path, output_path: Path):
    """转换为 Windows .ico 格式（包含多个尺寸）"""
    from PIL import Image
    
    print(f"🔄 转换 Windows 图标：{source_path.name} -> {output_path.name}")
    
    img = Image.open(source_path)
    
    # Windows ICO 支持的标准尺寸
    sizes = [16, 32, 48, 64, 128, 256]
    
    # 调整并保存为 ICO
    img.save(output_path, format='ICO', sizes=[(s, s) for s in sizes])
    print(f"✅ Windows 图标已保存：{output_path}")

def convert_to_icns(source_path: Path, output_path: Path):
    """转换为 macOS .icns 格式"""
    from PIL import Image
    
    print(f"🔄 转换 macOS 图标：{source_path.name} -> {output_path.name}")
    
    # macOS iconset 需要的尺寸
    iconset_dir = source_path.parent / "icon.iconset"
    iconset_dir.mkdir(exist_ok=True)
    
    sizes = {
        'icon_16x16.png': 16,
        'icon_16x16@2x.png': 32,
        'icon_32x32.png': 32,
        'icon_32x32@2x.png': 64,
        'icon_64x64.png': 64,
        'icon_128x128.png': 128,
        'icon_128x128@2x.png': 256,
        'icon_256x256.png': 256,
        'icon_256x256@2x.png': 512,
        'icon_512x512.png': 512,
        'icon_512x512@2x.png': 1024,
    }
    
    img = Image.open(source_path)
    
    for filename, size in sizes.items():
        resized = img.resize((size, size), Image.Resampling.LANCZOS)
        resized.save(iconset_dir / filename)
    
    # 使用 iconutil 转换为 .icns (仅 macOS)
    if sys.platform == 'darwin':
        try:
            subprocess.run(['iconutil', '-c', 'icns', str(iconset_dir), '-o', str(output_path)], check=True)
            print(f"✅ macOS 图标已保存：{output_path}")
        except subprocess.CalledProcessError as e:
            print(f"⚠️ iconutil 失败：{e}")
            print(f"   iconset 已保存在：{iconset_dir}")
            print(f"   手动运行：iconutil -c icns {iconset_dir} -o {output_path}")
            return False
        finally:
            # 清理临时文件
            import shutil
            shutil.rmtree(iconset_dir)
    else:
        print(f"⚠️ 非 macOS 系统，无法生成 .icns")
        print(f"   iconset 已保存在：{iconset_dir}")
        print(f"   在 macOS 上运行：iconutil -c icns {iconset_dir} -o {output_path}")
        return False
    
    return True

def convert_to_png(source_path: Path, output_path: Path, size: int = 512):
    """转换为 PNG 格式（用于 Linux）"""
    from PIL import Image
    
    print(f"🔄 转换 PNG 图标：{source_path.name} -> {output_path.name}")
    
    img = Image.open(source_path)
    resized = img.resize((size, size), Image.Resampling.LANCZOS)
    
    # 保存为 PNG（带透明通道）
    if resized.mode in ('RGBA', 'LA'):
        resized.save(output_path, format='PNG')
    else:
        # 转换为 RGBA 以支持透明
        resized = resized.convert('RGBA')
        resized.save(output_path, format='PNG')
    
    print(f"✅ PNG 图标已保存：{output_path}")

def find_source_image():
    """查找源图片"""
    extensions = ['.png', '.jpg', '.jpeg', '.webp', '.bmp']
    
    # 优先查找 source.* 文件
    for ext in extensions:
        source = ICONS_DIR / f"source{ext}"
        if source.exists():
            return source
    
    # 查找 catc.* 文件
    for ext in extensions:
        source = ICONS_DIR / f"catc{ext}"
        if source.exists():
            return source
    
    # 查找任何图片文件
    for ext in extensions:
        for file in ICONS_DIR.glob(f"*{ext}"):
            if file.name not in ['catc.png', 'catc.ico', 'catc.icns']:
                return file
    
    return None

def main():
    print("=" * 50)
    print("CatLang 图标转换工具")
    print("=" * 50)
    
    if not check_dependencies():
        sys.exit(1)
    
    # 创建 icons 目录
    ICONS_DIR.mkdir(exist_ok=True)
    
    # 查找源图片
    source = find_source_image()
    if not source:
        print("❌ 未找到源图片！")
        print(f"   请将图标图片放在：{ICONS_DIR}/source.png")
        print(f"   或：{ICONS_DIR}/catc.png")
        sys.exit(1)
    
    print(f"📁 找到源图片：{source}")
    
    # 转换图标
    ico_path = ICONS_DIR / "catc.ico"
    icns_path = ICONS_DIR / "catc.icns"
    png_path = ICONS_DIR / "catc.png"
    
    try:
        # 转换为 ICO (Windows)
        convert_to_ico(source, ico_path)
        
        # 转换为 ICNS (macOS)
        convert_to_icns(source, icns_path)
        
        # 转换为 PNG (Linux)
        convert_to_png(source, png_path)
        
        print("\n" + "=" * 50)
        print("✅ 所有图标转换完成！")
        print("=" * 50)
        print(f"\n生成的文件:")
        print(f"  🪟 Windows: {ico_path}")
        print(f"  🍎 macOS:  {icns_path}")
        print(f"  🐧 Linux:  {png_path}")
        
    except Exception as e:
        print(f"\n❌ 转换失败：{e}")
        sys.exit(1)

if __name__ == "__main__":
    main()
