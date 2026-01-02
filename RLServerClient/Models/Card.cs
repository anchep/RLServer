using System;

namespace RLServerClient.Models
{
    public class Card
    {
        public int Id { get; set; }
        public string Code { get; set; } = string.Empty;
        public string Type { get; set; } = string.Empty;
        public int Duration { get; set; } // 时长（天）
        public decimal Price { get; set; }
        public string Status { get; set; } = string.Empty;
        public DateTime CreatedAt { get; set; }
        public DateTime? UsedAt { get; set; }
        public int? UserId { get; set; }
    }
}
