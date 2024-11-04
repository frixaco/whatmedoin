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
	"go.mongodb.org/mongo-driver/bson/primitive"
	"go.mongodb.org/mongo-driver/mongo"
	"go.mongodb.org/mongo-driver/mongo/options"
)

type Activity struct {
	ID       primitive.ObjectID `bson:"_id,omitempty" json:"id"`
	Platform string             `json:"platform"`
	Title    string             `json:"title"`
	Url      string             `json:"url"`
	Date     string             `json:"date"`
}

func isValidPlatform(platform string) bool {
	return platform == "browser" || platform == "mobile" || platform == "windows" || platform == "macos"
}

func getLatestActivity(db *mongo.Collection, act *Activity) error {
	opts := options.FindOne().SetSort(bson.D{{Key: "_id", Value: -1}})
	err := db.FindOne(context.TODO(), bson.D{}, opts).Decode(act)
	if err != nil {
		return err
	}
	return nil
}

func saveActivity(db *mongo.Collection, act *Activity) error {
	result, err := db.InsertOne(context.TODO(), act)
	if err != nil {
		return err
	}

	act.ID = result.InsertedID.(primitive.ObjectID)
	return nil
}

func cleanUpActivities(db *mongo.Collection, la *Activity) error {
	err := getLatestActivity(db, la)
	if err != nil {
		return err
	}

	filter := bson.M{
		"_id": bson.M{
			"$ne": la.ID,
		},
	}
	db.DeleteMany(context.TODO(), filter)

	return nil
}

var mongoClient *mongo.Client

func main() {
	dotenv.Load()

	ctx, cancel := context.WithTimeout(context.TODO(), 10*time.Second)
	defer cancel()

	_client, err := mongo.Connect(ctx, options.Client().ApplyURI(dotenv.GetString("DATABASE_URL")))
	if err != nil {
		log.Fatal(err)
	}
	defer _client.Disconnect(ctx)
	mongoClient = _client

	e := echo.New()

	e.Use(middleware.LoggerWithConfig(middleware.LoggerConfig{
		Format: "method=${method}, uri=${uri}, status=${status}\n",
	}))
	e.Use(middleware.CORS())

	e.GET("/activity", func(c echo.Context) error {
		var activity Activity
		db := mongoClient.Database("test").Collection("activities")
		err := getLatestActivity(db, &activity)
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
		activity.Date = time.Now().UTC().Format("1/2/2006, 3:04:05 PM")

		if !isValidPlatform(activity.Platform) {
			return c.JSON(http.StatusBadRequest, map[string]string{"error": "invalid platform"})
		}

		db := mongoClient.Database("test").Collection("activities")
		saveActivity(db, &activity)

		return c.JSON(http.StatusCreated, activity)
	})

	e.DELETE("/activity", func(c echo.Context) error {
		db := mongoClient.Database("test").Collection("activities")
		var lastActivity Activity
		err := cleanUpActivities(db, &lastActivity)
		if err != nil {
			c.JSON(http.StatusInternalServerError, map[string]string{"error": err.Error()})
		}

		return c.JSON(http.StatusOK, lastActivity)
	})

	e.Logger.Fatal(e.Start(":3000"))
}
