namespace RLServerClient.Models
{
    public class Software
    {
        public int Id { get; set; }
        public string Name { get; set; } = string.Empty;
        public string ChineseName { get; set; } = string.Empty;
        public string Description { get; set; } = string.Empty;
        public string Details { get; set; } = string.Empty;
        public string ExecutableName { get; set; } = string.Empty;
        public string Version { get; set; } = string.Empty;
        public string StartupArguments { get; set; } = string.Empty;
        public bool IsActive { get; set; }
    }
}
