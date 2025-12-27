#!/usr/bin/env python3
import psycopg2
import random
import string
from datetime import datetime

# 数据库连接配置
DB_CONFIG = {
    'host': 'localhost',
    'port': 5432,
    'database': 'rl_server',
    'user': 'admin',
    'password': 'password'
}

# 生成随机卡密
def generate_card_code(length=16):
    characters = string.ascii_uppercase + string.digits
    code = ''.join(random.choice(characters) for _ in range(length))
    return f"RC-{code[:8]}-{code[8:]}"

# 生成10张卡密并插入数据库
def generate_and_insert_cards(count=10):
    try:
        # 连接数据库
        conn = psycopg2.connect(**DB_CONFIG)
        cursor = conn.cursor()
        
        # 生成并插入卡密
        for _ in range(count):
            card_code = generate_card_code()
            amount = 99  # 默认金额
            vip_level = 1  # 默认VIP等级
            duration_days = 30  # 默认有效期30天
            
            # 插入数据
            sql = """
            INSERT INTO recharge_cards (card_code, amount, vip_level, duration_days, is_used)
            VALUES (%s, %s, %s, %s, %s)
            """
            cursor.execute(sql, (card_code, amount, vip_level, duration_days, False))
            
            print(f"生成卡密: {card_code}，VIP等级: {vip_level}，有效期: {duration_days}天")
        
        # 提交事务
        conn.commit()
        
        # 关闭连接
        cursor.close()
        conn.close()
        
        print(f"\n成功生成并插入 {count} 张卡密到数据库!")
        
    except Exception as e:
        print(f"生成卡密时出错: {e}")

if __name__ == "__main__":
    generate_and_insert_cards(10)
