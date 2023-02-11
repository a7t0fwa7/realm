// Code generated by github.com/99designs/gqlgen, DO NOT EDIT.

package generated

import (
	"context"

	"github.com/99designs/gqlgen/graphql"
	"github.com/kcarretto/realm/tavern/graphql/models"
)

// region    ************************** generated!.gotpl **************************

// endregion ************************** generated!.gotpl **************************

// region    ***************************** args.gotpl *****************************

// endregion ***************************** args.gotpl *****************************

// region    ************************** directives.gotpl **************************

// endregion ************************** directives.gotpl **************************

// region    **************************** field.gotpl *****************************

// endregion **************************** field.gotpl *****************************

// region    **************************** input.gotpl *****************************

func (ec *executionContext) unmarshalInputClaimTasksInput(ctx context.Context, obj interface{}) (models.ClaimTasksInput, error) {
	var it models.ClaimTasksInput
	asMap := map[string]interface{}{}
	for k, v := range obj.(map[string]interface{}) {
		asMap[k] = v
	}

	fieldsInOrder := [...]string{"principal", "hostname", "sessionIdentifier", "hostIdentifier", "agentIdentifier"}
	for _, k := range fieldsInOrder {
		v, ok := asMap[k]
		if !ok {
			continue
		}
		switch k {
		case "principal":
			var err error

			ctx := graphql.WithPathContext(ctx, graphql.NewPathWithField("principal"))
			it.Principal, err = ec.unmarshalNString2string(ctx, v)
			if err != nil {
				return it, err
			}
		case "hostname":
			var err error

			ctx := graphql.WithPathContext(ctx, graphql.NewPathWithField("hostname"))
			it.Hostname, err = ec.unmarshalNString2string(ctx, v)
			if err != nil {
				return it, err
			}
		case "sessionIdentifier":
			var err error

			ctx := graphql.WithPathContext(ctx, graphql.NewPathWithField("sessionIdentifier"))
			it.SessionIdentifier, err = ec.unmarshalNString2string(ctx, v)
			if err != nil {
				return it, err
			}
		case "hostIdentifier":
			var err error

			ctx := graphql.WithPathContext(ctx, graphql.NewPathWithField("hostIdentifier"))
			it.HostIdentifier, err = ec.unmarshalNString2string(ctx, v)
			if err != nil {
				return it, err
			}
		case "agentIdentifier":
			var err error

			ctx := graphql.WithPathContext(ctx, graphql.NewPathWithField("agentIdentifier"))
			it.AgentIdentifier, err = ec.unmarshalNString2string(ctx, v)
			if err != nil {
				return it, err
			}
		}
	}

	return it, nil
}

func (ec *executionContext) unmarshalInputSubmitTaskResultInput(ctx context.Context, obj interface{}) (models.SubmitTaskResultInput, error) {
	var it models.SubmitTaskResultInput
	asMap := map[string]interface{}{}
	for k, v := range obj.(map[string]interface{}) {
		asMap[k] = v
	}

	fieldsInOrder := [...]string{"taskID", "execStartedAt", "execFinishedAt", "output", "error"}
	for _, k := range fieldsInOrder {
		v, ok := asMap[k]
		if !ok {
			continue
		}
		switch k {
		case "taskID":
			var err error

			ctx := graphql.WithPathContext(ctx, graphql.NewPathWithField("taskID"))
			it.TaskID, err = ec.unmarshalNID2int(ctx, v)
			if err != nil {
				return it, err
			}
		case "execStartedAt":
			var err error

			ctx := graphql.WithPathContext(ctx, graphql.NewPathWithField("execStartedAt"))
			it.ExecStartedAt, err = ec.unmarshalNTime2timeᚐTime(ctx, v)
			if err != nil {
				return it, err
			}
		case "execFinishedAt":
			var err error

			ctx := graphql.WithPathContext(ctx, graphql.NewPathWithField("execFinishedAt"))
			it.ExecFinishedAt, err = ec.unmarshalOTime2ᚖtimeᚐTime(ctx, v)
			if err != nil {
				return it, err
			}
		case "output":
			var err error

			ctx := graphql.WithPathContext(ctx, graphql.NewPathWithField("output"))
			it.Output, err = ec.unmarshalNString2string(ctx, v)
			if err != nil {
				return it, err
			}
		case "error":
			var err error

			ctx := graphql.WithPathContext(ctx, graphql.NewPathWithField("error"))
			it.Error, err = ec.unmarshalOString2ᚖstring(ctx, v)
			if err != nil {
				return it, err
			}
		}
	}

	return it, nil
}

// endregion **************************** input.gotpl *****************************

// region    ************************** interface.gotpl ***************************

// endregion ************************** interface.gotpl ***************************

// region    **************************** object.gotpl ****************************

// endregion **************************** object.gotpl ****************************

// region    ***************************** type.gotpl *****************************

func (ec *executionContext) unmarshalNClaimTasksInput2githubᚗcomᚋkcarrettoᚋrealmᚋtavernᚋgraphqlᚋmodelsᚐClaimTasksInput(ctx context.Context, v interface{}) (models.ClaimTasksInput, error) {
	res, err := ec.unmarshalInputClaimTasksInput(ctx, v)
	return res, graphql.ErrorOnPath(ctx, err)
}

func (ec *executionContext) unmarshalNSubmitTaskResultInput2githubᚗcomᚋkcarrettoᚋrealmᚋtavernᚋgraphqlᚋmodelsᚐSubmitTaskResultInput(ctx context.Context, v interface{}) (models.SubmitTaskResultInput, error) {
	res, err := ec.unmarshalInputSubmitTaskResultInput(ctx, v)
	return res, graphql.ErrorOnPath(ctx, err)
}

// endregion ***************************** type.gotpl *****************************