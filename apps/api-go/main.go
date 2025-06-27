package main

import (
	"bufio"
	"encoding/json"
	"fmt"
	"net/http"
	"os"
	"strings"
	"time"

	"github.com/labstack/echo/v4"
	"github.com/labstack/echo/v4/middleware"
)

type Activity struct {
	Platform string    `json:"platform"`
	Title    string    `json:"title"`
	Url      string    `json:"url"`
	Date     time.Time `json:"date"`
}

func isValidPlatform(platform string) bool {
	return platform == "browser" || platform == "mobile" || platform == "windows" || platform == "macos"
}

func getLatestActivity(c echo.Context, act *Activity) error {
	file, err := os.Open("/data/activities.jsonl")
	if err != nil {
		if os.IsNotExist(err) {
			return fmt.Errorf("no activities found")
		}
		return err
	}
	defer file.Close()

	var lastLine string
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := strings.TrimSpace(scanner.Text())
		if line != "" {
			lastLine = line
		}
	}

	if err := scanner.Err(); err != nil {
		return err
	}

	if lastLine == "" {
		return fmt.Errorf("no activities found")
	}

	err = json.Unmarshal([]byte(lastLine), act)
	if err != nil {
		return err
	}

	return nil
}

func saveActivity(c echo.Context, act *Activity) error {
	fmt.Println("Saving activity with date:", act.Date)

	// Marshal activity to JSON
	data, err := json.Marshal(act)
	if err != nil {
		return err
	}

	// Open file in append mode, create if doesn't exist
	file, err := os.OpenFile("/data/activities.jsonl", os.O_APPEND|os.O_CREATE|os.O_WRONLY, 0644)
	if err != nil {
		return err
	}
	defer file.Close()

	// Append JSON line
	_, err = file.WriteString(string(data) + "\n")
	if err != nil {
		return err
	}

	return nil
}

func cleanUpActivities(c echo.Context, la *Activity) error {
	err := getLatestActivity(c, la)
	if err != nil {
		return err
	}

	// Marshal latest activity to JSON
	data, err := json.Marshal(la)
	if err != nil {
		return err
	}

	// Overwrite file with just the latest activity
	err = os.WriteFile("/data/activities.jsonl", []byte(string(data)+"\n"), 0644)
	if err != nil {
		return err
	}

	return nil
}

func main() {
	e := echo.New()

	e.Use(middleware.LoggerWithConfig(middleware.LoggerConfig{
		Format: "method=${method}, uri=${uri}, status=${status}\n",
	}))
	e.Use(middleware.CORS())

	e.GET("/activity", func(c echo.Context) error {
		var activity Activity

		err := getLatestActivity(c, &activity)
		if err != nil {
			return c.JSON(http.StatusNotFound, map[string]string{"error": err.Error()})
		}
		return c.JSON(http.StatusOK, activity)
	})

	e.POST("/activity", func(c echo.Context) error {
		var activity Activity
		err := c.Bind(&activity)
		if err != nil {
			return c.JSON(http.StatusBadRequest, map[string]string{"error": err.Error()})
		}

		// Set the current time with full timestamp precision
		now := time.Now().UTC()
		fmt.Println("Setting activity time to:", now)
		activity.Date = now

		if !isValidPlatform(activity.Platform) {
			return c.JSON(http.StatusBadRequest, map[string]string{"error": "invalid platform"})
		}

		err = saveActivity(c, &activity)
		if err != nil {
			return c.JSON(http.StatusInternalServerError, map[string]string{"error": err.Error()})
		}

		return c.JSON(http.StatusCreated, activity)
	})

	e.DELETE("/activity", func(c echo.Context) error {
		var lastActivity Activity
		err := cleanUpActivities(c, &lastActivity)
		if err != nil {
			return c.JSON(http.StatusInternalServerError, map[string]string{"error": err.Error()})
		}

		return c.JSON(http.StatusOK, lastActivity)
	})

	e.Logger.Fatal(e.Start(":3000"))
}
