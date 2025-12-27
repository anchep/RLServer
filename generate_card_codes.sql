-- 生成10张卡密的SQL脚本
DO $$
DECLARE
    i INT := 0;
    card_code VARCHAR(20);
BEGIN
    WHILE i < 10 LOOP
        -- 生成随机卡密
        card_code := 'RC-' || 
                     substr(md5(random()::text), 1, 8) || '-' || 
                     substr(md5(random()::text), 1, 8);
        
        -- 插入卡密数据
        INSERT INTO recharge_cards (card_code, amount, vip_level, duration_days, is_used)
        VALUES (card_code, 99, 1, 30, FALSE);
        
        -- 打印生成的卡密
        RAISE NOTICE '生成卡密: %，VIP等级: 1，有效期: 30天', card_code;
        
        i := i + 1;
    END LOOP;
END $$;
