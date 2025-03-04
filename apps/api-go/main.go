package main

import (
	"context"
	"fmt"
	"net/http"
	"os"
	"time"

	"github.com/jackc/pgx/v5"
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

func getLatestActivity(c echo.Context, db *pgx.Conn, act *Activity) error {
	err := db.QueryRow(context.Background(), "select * from activities order by id desc limit 1").Scan(&act.Id, &act.Platform, &act.Title, &act.Url, &act.Date)
	if err != nil {
		return err
	}
	return nil
}

func saveActivity(c echo.Context, db *pgx.Conn, act *Activity) error {
	fmt.Println("Saving activity", act.Date)
	_, err := db.Exec(context.Background(), "insert into activities (platform, title, url, date) values ($1, $2, $3, $4) returning id", act.Platform, act.Title, act.Url, act.Date)
	if err != nil {
		return err
	}

	return nil
}

func cleanUpActivities(c echo.Context, db *pgx.Conn, la *Activity) error {
	err := getLatestActivity(c, db, la)
	if err != nil {
		return err
	}

	_, err = db.Exec(context.Background(), "delete from activities where id = $1", la.Id)
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

	dbConn, err := pgx.Connect(context.Background(), dbUrl)
	if err != nil {
		fmt.Fprintf(os.Stderr, "Unable to connect to database: %v\n", err)
		os.Exit(1)
	}
	defer dbConn.Close(context.Background())

	e := echo.New()

	e.Use(middleware.LoggerWithConfig(middleware.LoggerConfig{
		Format: "method=${method}, uri=${uri}, status=${status}\n",
	}))
	e.Use(middleware.CORS())

	e.GET("/activity", func(c echo.Context) error {
		var activity Activity

		err := getLatestActivity(c, dbConn, &activity)
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
		activity.Date = time.Now().UTC()

		if !isValidPlatform(activity.Platform) {
			return c.JSON(http.StatusBadRequest, map[string]string{"error": "invalid platform"})
		}

		saveActivity(c, dbConn, &activity)

		return c.JSON(http.StatusCreated, activity)
	})

	e.DELETE("/activity", func(c echo.Context) error {
		var lastActivity Activity
		err := cleanUpActivities(c, dbConn, &lastActivity)
		if err != nil {
			c.JSON(http.StatusInternalServerError, map[string]string{"error": err.Error()})
		}

		return c.JSON(http.StatusOK, lastActivity)
	})

	e.Logger.Fatal(e.Start(":3000"))
}
