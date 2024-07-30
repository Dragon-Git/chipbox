/*
Copyright © 2024 NAME HERE <EMAIL ADDRESS>
*/
package cmd

import (
	"fmt"

	"github.com/spf13/cobra"
)

// simCmd represents the sim command
var simCmd = &cobra.Command{
	Use:   "sim",
	Short: "sim use vcs",
	Long: `A longer description that spans multiple lines and likely contains examples
and usage of using your command. For example:

Cobra is a CLI library for Go that empowers applications.
This application is a tool to generate the needed files
to quickly create a Cobra application.`,
	Run: func(cmd *cobra.Command, args []string) {
		fmt.Println("sim called")
	},
}

func init() {
	rootCmd.AddCommand(simCmd)

	// Here you will define your flags and configuration settings.

	// Cobra supports Persistent Flags which will work for this command
	// and all subcommands, e.g.:
	// simCmd.PersistentFlags().String("foo", "", "A help for foo")

	// Cobra supports local flags which will only run when this command
	// is called directly, e.g.:
	simCmd.Flags().BoolP("fpga", "f", false, "Help message for toggle")
	simCmd.Flags().BoolP("upf", "u", false, "Help message for toggle")
	simCmd.Flags().BoolP("gls", "g", false, "Help message for toggle")
	simCmd.Flags().BoolP("pg", "p", false, "Help message for toggle")
}
