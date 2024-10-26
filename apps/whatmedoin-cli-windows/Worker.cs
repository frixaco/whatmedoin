namespace whatmedoin_cli_windows;
using System.Diagnostics;
using System.Runtime.InteropServices;
using System.Text;

public class Worker : BackgroundService
{
    private readonly ILogger<Worker> _logger;

    public Worker(ILogger<Worker> logger)
    {
        _logger = logger;
    }
    
    [DllImport("user32.dll")]
    private static extern IntPtr GetForegroundWindow();
    
    [DllImport("user32.dll")]
    private static extern int GetWindowText(IntPtr hWnd, StringBuilder lpString, int nMaxCount);
    
    [DllImport("user32.dll")]
    private static extern IntPtr GetWindowThreadProcessId(IntPtr hWnd, out int lpdwProcessId);

    public static (string title, string processName, string processPath) GetForegroundWindowInfo()
    {
        var windowHandle = GetForegroundWindow();
        var titleBuilder = new StringBuilder(256);

        GetWindowText(windowHandle, titleBuilder, titleBuilder.Capacity);
        GetWindowThreadProcessId(windowHandle, out var processId);
        
        var process = Process.GetProcessById(processId);

        return
        (

            titleBuilder.ToString(),
            process.ProcessName,
            process.MainModule?.FileName ?? string.Empty
        );
    }
    
    // public SendActivity() {}

    protected override async Task ExecuteAsync(CancellationToken stoppingToken)
    {
        while (!stoppingToken.IsCancellationRequested)
        {
            if (_logger.IsEnabled(LogLevel.Information))
            {
                var (title, processName, processPath) = GetForegroundWindowInfo();
                _logger.LogInformation(
                    "Foreground window title: {title}, process: {processName}, processPath: {processPath}",
                    title, processName, processPath
                    );
            }

            await Task.Delay(5000, stoppingToken);
        }
    }
}