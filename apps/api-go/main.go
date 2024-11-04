package main

import (
	"context"
	"log"
	"net/http"
	"time"

	"github.com/labstack/echo/v4"
	"github.com/labstack/echo/v4/middleware"
	"github.com/profclems/go-dotenv"
	"go.mongodb.org/mongo-driver/bson"
	"go.mongodb.org/mongo-driver/mongo"
	"go.mongodb.org/mongo-driver/mongo/options"
)

type Activity struct {
	Platform string `json:"platform"`
	Title    string `json:"title"`
	Url      string `json:"url"`
	Date     string `json:"date"`
}

func isValidPlatform(platform string) bool {
	return platform == "browser" || platform == "mobile" || platform == "windows" || platform == "macos"
}

func getActivity(ctx echo.Context) error {
	var activity Activity
	database := mongoClient.Database("test").Collection("activities")
	opts := options.FindOne().SetSort(bson.D{{Key: "_id", Value: -1}})
	err := database.FindOne(context.Background(), bson.D{}, opts).Decode(&activity)
	if err != nil {
		return ctx.JSON(http.StatusNotFound, map[string]string{"error": err.Error()})
	}
	return ctx.JSON(http.StatusOK, activity)
}

func postActivity(ctx echo.Context) error {
	var activity Activity
	err := ctx.Bind(&activity)
	if err != nil {
		return ctx.JSON(http.StatusBadRequest, map[string]string{"error": err.Error()})
	}

	if !isValidPlatform(activity.Platform) {
		return ctx.JSON(http.StatusBadRequest, map[string]string{"error": "invalid platform"})
	}

	activity.Date = time.Now().UTC().Format("1/2/2006, 3:04:05 PM")
	database := mongoClient.Database("test").Collection("activities")
	database.InsertOne(context.Background(), activity)
	return ctx.JSON(http.StatusCreated, activity)
}

var mongoClient *mongo.Client

func main() {
	dotenv.Load()

	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()

	client, err := mongo.Connect(ctx, options.Client().ApplyURI(dotenv.GetString("DATABASE_URL")))
	if err != nil {
		log.Fatal(err)
	}
	defer client.Disconnect(ctx)
	mongoClient = client

	e := echo.New()
	e.Use(middleware.LoggerWithConfig(middleware.LoggerConfig{
		Format: "method=${method}, uri=${uri}, status=${status}\n",
	}))
	e.Use(middleware.CORS())
	e.GET("/activity", getActivity)
	e.POST("/activity", postActivity)
	e.Logger.Fatal(e.Start(":3000"))
}
