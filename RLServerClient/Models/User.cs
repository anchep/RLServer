using System;
using Newtonsoft.Json;

namespace RLServerClient.Models
{
    public class User
    {
        public int Id { get; set; }
        public string Username { get; set; } = string.Empty;
        public string Email { get; set; } = string.Empty;
        
        [JsonProperty("vip_level")]
        public int VipLevel { get; set; }
        
        [JsonProperty("vip_end_time")]
        public DateTime? VipEndTime { get; set; }
        
        [JsonProperty("created_at")]
        public DateTime CreatedAt { get; set; }
        
        [JsonProperty("updated_at")]
        public DateTime UpdatedAt { get; set; }
        
        [JsonProperty("is_active")]
        public bool IsActive { get; set; }
    }
}
