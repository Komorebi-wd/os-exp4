import os

#在保留原始链接器脚本的同时，
# 创建一个修改过的版本，
# 其中每个应用程序的起始内存地址都是唯一的。
# 这样做可以确保在编译不同的应用程序时，
# 它们不会互相干扰，
# 因为它们被分配到了不同的内存区域。
base_address = 0x80400000
step = 0x20000
linker = 'src/linker.ld'

app_id = 0 
apps = os.listdir('src/bin')#获取目录下所有文件名
apps.sort()#排序
for app in apps:
    app = app[:app.find('.')]
    lines = []#记录修改后的数据
    lines_before = []#记录修改前的数据
    with open(linker, 'r') as f:#以只读方式打开文件
        for line in f.readlines():
            lines_before.append(line)#记录修改前的数据
            line = line.replace(hex(base_address), hex(base_address+step*app_id))#修改地址，为原有基础地址加上偏移量
            lines.append(line)#记录修改后的数据
    with open(linker, 'w+') as f:
        f.writelines(lines)
    os.system('cargo build --bin %s --release' % app)#编译
    print('[build.py] application %s start with address %s' %(app, hex(base_address+step*app_id)))
    with open(linker, 'w+') as f:
        f.writelines(lines_before)
    app_id = app_id + 1
