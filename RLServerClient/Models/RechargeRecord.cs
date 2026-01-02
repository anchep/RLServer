using System;

namespace RLServerClient.Models
{
    public class RechargeRecord
    {
        public string CardCode { get; set; } = string.Empty;
        public int Amount { get; set; }
        public DateTime CreatedAt { get; set; }
    }
}