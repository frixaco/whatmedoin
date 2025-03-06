package main

import (
	"context"
	"fmt"
	"net/http"
	"os"
	"time"

	"github.com/jackc/pgx/v5/pgxpool"
	"github.com/labstack/echo/v4"
	"github.com/labstack/echo/v4/middleware"
	"github.com/profclems/go-dotenv"
)

type Activity struct {
	Id       int64     `json:"id"`
	Platform string    `json:"platform"`
	Title    string    `json:"title"`
	Url      string    `json:"url"`
	Date     time.Time `json:"date"`
}

func isValidPlatform(platform string) bool {
	return platform == "browser" || platform == "mobile" || platform == "windows" || platform == "macos"
}

func getLatestActivity(c echo.Context, pool *pgxpool.Pool, act *Activity) error {
	ctx := c.Request().Context()
	err := pool.QueryRow(ctx, "select id, platform, title, url, date AT TIME ZONE 'UTC' from activities order by id desc limit 1").Scan(&act.Id, &act.Platform, &act.Title, &act.Url, &act.Date)
	if err != nil {
		return err
	}
	return nil
}

func saveActivity(c echo.Context, pool *pgxpool.Pool, act *Activity) error {
	ctx := c.Request().Context()
	fmt.Println("Saving activity with date:", act.Date)
	_, err := pool.Exec(ctx, "insert into activities (platform, title, url, date) values ($1, $2, $3, $4::timestamp) returning id", act.Platform, act.Title, act.Url, act.Date)
	if err != nil {
		return err
	}
	return nil
}

func cleanUpActivities(c echo.Context, pool *pgxpool.Pool, la *Activity) error {
	ctx := c.Request().Context()
	err := getLatestActivity(c, pool, la)
	if err != nil {
		return err
	}

	_, err = pool.Exec(ctx, "delete from activities where id != $1", la.Id)
	if err != nil {
		return err
	}

	return nil
}

func main() {
	dotenv.SetConfigFile(".env")
	err := dotenv.Load()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error loading .env file: %v\n", err)
	}

	dbUrl := dotenv.GetString("DATABASE_URL")
	if dbUrl == "" {
		fmt.Fprintf(os.Stderr, "DATABASE_URL environment variable is not set\n")
		os.Exit(1)
	}

	pool, err := pgxpool.New(context.Background(), os.Getenv("DATABASE_URL"))
	if err != nil {
		fmt.Fprintf(os.Stderr, "Unable to create connection pool: %v\n", err)
		os.Exit(1)
	}
	defer pool.Close()

	e := echo.New()

	e.Use(middleware.LoggerWithConfig(middleware.LoggerConfig{
		Format: "method=${method}, uri=${uri}, status=${status}\n",
	}))
	e.Use(middleware.CORS())

	e.GET("/activity", func(c echo.Context) error {
		var activity Activity

		err := getLatestActivity(c, pool, &activity)
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

		err = saveActivity(c, pool, &activity)
		if err != nil {
			return c.JSON(http.StatusInternalServerError, map[string]string{"error": err.Error()})
		}

		return c.JSON(http.StatusCreated, activity)
	})

	e.DELETE("/activity", func(c echo.Context) error {
		var lastActivity Activity
		err := cleanUpActivities(c, pool, &lastActivity)
		if err != nil {
			return c.JSON(http.StatusInternalServerError, map[string]string{"error": err.Error()})
		}

		return c.JSON(http.StatusOK, lastActivity)
	})

	e.Logger.Fatal(e.Start(":3000"))
}
